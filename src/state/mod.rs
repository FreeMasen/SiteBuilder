use std::{
    path::{Path, PathBuf},
    io::{Read, Write},
    fs::{File, DirBuilder, remove_file, OpenOptions},
    env::{var_os},
};

use bincode::{deserialize_from, serialize_into};
use pulldown_cmark::{Parser, html::push_html};

pub mod site_state;
pub mod website;
pub mod project;
pub mod valid;
pub mod error;
pub mod fonts;
pub mod image;
pub mod meta;
pub mod build;

pub use site_state::SiteState;
pub use website::Website;
pub use error::{StateError, StateResult};
pub use fonts::Fonts;
pub use image::Image;
pub use meta::Meta;
pub use valid::Valid;
pub use project::Project;

use messaging::ServerMessage;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub site_options: Vec<CachedSite>,
    selected_idx: Option<usize>,
    pub site: Option<SiteState>,
    pub message: Option<ServerMessage>,
    pub current_view: Route,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CachedSite {
    pub id: u32,
    pub title: String,
    pub path: PathBuf,
}

impl State {
    pub fn get() -> Result<State, StateError> {
        let f = Self::get_cache_file()?;
        let mut site_options: Vec<CachedSite> = if let Ok(opts) = deserialize_from(f) {
            opts
        } else {
            vec!()
        };
        site_options.retain(|o| o.path.exists());
        Ok(State {
            site_options,
            selected_idx: None,
            site: None,
            message: None,
            current_view: Route::Select,
        })
    }

    pub fn cache(&self) -> Result<(), StateError> {
        let f = Self::get_cache_file()?;
        let ret = serialize_into(f, &self.site_options)?;
        if let Some(ref s) = self.site {
            s.cache()
        }
        Ok(ret)
    }

    pub fn choose_site(&mut self, idx: usize) -> Result<(), StateError> {
        let site = if idx > self.site_options.len() {
            SiteState::default()
        } else {
            SiteState::get(&self.site_options[idx].path)?.clone()
        };
        self.site = Some(site);
        self.current_view = Route::All;
        self.selected_idx = Some(idx);
        Ok(())
    }

    /// Add a new site to the global site's cache
    pub fn add_site(&mut self, path: PathBuf) {
        //First, get the new id for this site in case we need it for the
        //new site's title
        let id = self.site_options.iter().map(|s| s.id).max().unwrap_or(0);
        let new_site = if let Ok(ss) = SiteState::get(&path) {
            //If the folder already has a .site_builder file with a valid
            //site state then we will just use that
            ss
        } else {
            //If no .site_builder file is found we create a new one
            //with a default title of Site-{id} and the path provided
            let s = SiteState::new(&format!("Site-{}", id), &path);
            //Cache the site to make sure it is saved with this new data
            s.cache();
            s
        };
        
        let idx = if let Some(idx) = self.site_options.iter().position(|s| s.path == path) {
            // If the site already exists in our cache, we can just use the index
            // to set the self.selected_idx property
            idx
        } else {
            //If the site doesn't already exist, we can create a new
            //SiteCache with out id, the title created above and 
            //the path provided
            let new_cache = CachedSite {
                id,
                title: new_site.website.title.clone(),
                path: path,
            };
            //Add this to our cache
            self.site_options.push(new_cache);
            //use the last index in our site_options
            //as the selected index
            self.site_options.len() - 1
        };
        self.selected_idx = Some(idx);
        self.current_view = Route::All;
        self.site = Some(new_site);
    }

    pub fn add_message<T: ToString>(&mut self, content: T, is_error: bool) {
        if is_error {
            println!("Error: {}", &content.to_string());
        }
        self.message = Some(ServerMessage {
            content: content.to_string(),
            is_error,
        });
    }

    pub fn clear_message(&mut self) {
        self.message = None;
    }

    pub fn update_site_title(&mut self, title: String) -> StateResult {
        {
            let s = self.site()?;
            s.update_title(title.clone());
            s.cache()
        }
        self.update_cached_site(title)?;
        Ok(String::from("Successfully updated site title"))
    }

    pub fn update_cached_site(&mut self, title: String) -> Result<(), StateError> {
        let idx = if let Some(idx) = self.selected_idx {
            idx.clone()
        } else {
            return Err(StateError::new("Unable to get selected index"));
        };
        if let Some(ref mut c) = self.site_options.get_mut(idx) {
            c.title = title;
        };
        Ok(())
    }

    pub fn update_site(&mut self) -> StateResult {
        let s = self.site()?;
        return s.update_from_source()
    }

    pub fn site_valid(&mut self) -> StateResult {
        let s = self.site()?;
        s.is_valid()
    }

    pub fn build_site(&mut self) -> StateResult {
        let s = self.site()?;
        s.build()
    }

    pub fn add_project(&mut self, name: String) -> StateResult {
        let s = self.site()?;
        s.add_project(name)
    }

    pub fn update_project(&mut self, project: Project) -> StateResult {
        {let s = self.site()?;
        s.update_project(project);
        s.write_input()?;
        s.selected_project = None;}
        self.current_view = Route::All;
        Ok(String::from("Successfully updated project"))
    }

    pub fn remove_project(&mut self) -> StateResult {
        {let s = self.site()?;
        let p = {
            let p = s.selected_project()?;
            p.clone()
        };
        s.website.delete_project(&p)?;}
        self.change_view(Route::All, None);
        Ok(String::from("Successfully deleted project"))
    }

    pub fn change_view(&mut self, route: Route, project: Option<Project>) {
        self.current_view = route;
        if let Some(ref mut s) = self.site {
            s.selected_project = project;
        }
    }

    pub fn source(&mut self) -> Option<&str> {
        if let Ok(s) = self.site() {
            s.source.to_str()
        } else {
            None
        }
    }

    pub fn add_font(&mut self, path: &PathBuf, bold: bool) -> StateResult {
        let s = self.site()?;
        s.add_font(path, bold)
    }

    pub fn remove_font(&mut self, bold: bool) -> StateResult {
        let s = self.site()?;
        let path_opt = if bold {
            s.website.fonts.bold.clone()
        } else {
            s.website.fonts.normal.clone()
        };
        let path = if let Some(p) = path_opt {
            p
        } else {
            return Ok(String::from("Font was not set"))
        };
        remove(&path)?;
        if bold {
            s.website.fonts.bold = None;
        } else {
            s.website.fonts.normal = None;
        }
        Ok(String::from("Successfully removed font"))
    }

    pub fn update_about(&mut self, content: String) -> StateResult {
        {let s = self.site()?;
        s.website.about = content;
        s.write_input()?;}
        self.change_view(Route::All, None);
        Ok(String::from("Successfully update about"))
    }

    pub fn update_about_image(&mut self, path: PathBuf) -> StateResult {
        let s = self.site()?;
        copy_file(&path, &s.source)?;
        s.website.image = path;
        Ok(String::from("Successfully update about image"))
    }

    pub fn add_project_image(&mut self, path: PathBuf) -> StateResult {
        let s = self.site()?;
        s.add_project_image(&path)?;
        Ok(String::from("Successfully added project image"))
    }

    pub fn remove_project_image(&mut self, path: PathBuf) -> StateResult {
        remove(&path)?;
        let s = self.site()?;
        s.update_from_source()?;
        Ok(String::from("Successfully removed project image"))
    }

    pub fn update_source(&mut self, path: PathBuf) -> StateResult {
        {
            let s = self.site()?;
            s.source = path;
            s.ensure_dir_defaults();
        }
        self.update_selected_source();
        Ok(String::from("Successfully updated source folder"))
    }

    fn update_selected_source(&mut self) {
        let site = if let Some(ref s) = self.site {
            s.clone()
        } else {
            return;
        };
        let idx = if let Some(i) = self.selected_idx {
            i
        } else {
            return;
        };
        if let Some(ref mut c) = self.site_options.get_mut(idx) {
            c.path = site.source.clone();
        }
    }

    pub fn update_dest(&mut self, path: PathBuf) -> StateResult {
        let s = self.site()?;
        s.destination = path;
        Ok(String::from("Successfully updated destination folder"))
    }

    fn site(&mut self) -> Result<&mut SiteState, StateError> {
        if let Some(ref mut s) = self.site {
            Ok(s)
        } else {
            Err(StateError::new("No site selected"))
        }
    }

    fn get_cache_file() -> Result<File, StateError> {
        let path = Self::get_home()?.join(".site_builder_cache");
        let f = OpenOptions::new().write(true).read(true).create(true).open(&path)?;
        Ok(f)
    }

    fn get_home() -> Result<PathBuf, StateError> {
        let arg = if cfg!(windows) {
            "USERPROFILE"
        } else {
            "HOME"
        };
        match var_os(arg) {
            Some(s) => Ok(PathBuf::from(s)),
            None => Err(StateError::new("Error getting user folder")),
        }
    }
}


/// extract the contents of content.md for editing
fn content(path: &Path) -> String {
    let mut buf = String::new();
    if let Ok(mut f) = File::open(path) {
        match f.read_to_string(&mut buf) {
            Ok(_size) => buf,
            Err(_) => String::new()
        }
    } else {
        String::new()
    }
}

/// Ensure a folder exists
pub fn ensure_folder(path: &PathBuf) -> ::std::io::Result<()> {
    if path.exists() {
        return Ok(())
    }
    let mut db = DirBuilder::new();
    db.recursive(true);
    db.create(path)
}

/// Write the contents to a file
pub fn write_file(content: &str, path: PathBuf) -> ::std::io::Result<()> {
    println!("Attempting to create file at {:?}", &path);
    let mut f = OpenOptions::new().read(true).write(true).create(true).open(&path)?;
    f.write_all(content.as_bytes())
}

pub fn copy_file(source: &PathBuf, dest_dir: &PathBuf) -> Result<PathBuf, String> {
    println!("copy_file {:?} to {:?}", source, dest_dir);
    if let Some(file_name) = source.file_name() {
        let mut dest = dest_dir.join(&file_name);
        let mut counter = 0;
        while dest.exists() {
            counter += 1;
            let fn_str = file_name.to_string_lossy();
            let new_fn = fn_str.replace(".", &format!("{}.", counter));
            dest.set_file_name(new_fn);
        }
        println!("copying file from {:?} to {:?}", &source, &dest);
        let mut i_f = File::open(source).map_err(map_e)?;
        let mut buf = vec!();
        i_f.read_to_end(&mut buf).map_err(map_e)?;
        let mut o_f = File::create(&dest).map_err(map_e)?;
        o_f.write_all(&mut buf).map_err(map_e)?;
        Ok(dest)
    } else {
        Err("Unable to get source's filename".into())
    }
}

pub fn remove(path: &PathBuf) -> Result<(), String> {
    remove_file(path).map_err(map_e)?;
    Ok(())
}

fn map_e(e: ::std::io::Error) -> String {
    format!("{:?}", e)
}

fn file_name(path: &PathBuf) -> String {
    if let Some(f_n) = path.file_name() {
        return f_n.to_string_lossy().to_string()
    }
    String::new()
}

fn generate_html(md: &String) -> String {
    let p = Parser::new(md);
    let mut ret = String::new();
    push_html(&mut ret, p);
    ret
}

fn get_cache_file(path: &PathBuf) -> Result<File, ::std::io::Error> {
    OpenOptions::new().write(true).read(true).create(true).open(path)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Route {
    All = 0,
    Project = 1,
    About = 2,
    Select = 3,
}