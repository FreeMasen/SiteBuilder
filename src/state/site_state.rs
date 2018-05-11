use std::{
    fs::{File, remove_dir_all, remove_file},
    path::{PathBuf},
};

use bincode::{serialize_into, deserialize_from};
use chrono::prelude::*;
use tera::{Tera, Context};
use toml;
use walkdir::WalkDir;

use project::{Project};
use error::{StateError, StateResult};
use website::Website;
use build::{IndexProject, Page};

const ABOUT: &'static str = include_str!("../assets/templates/about.html");
const BASE: &'static str = include_str!("../assets/templates/base.html");
const CONTACT: &'static str = include_str!("../assets/templates/contact.html");
const INDEX: &'static str = include_str!("../assets/templates/index.html");
const PAGE: &'static str = include_str!("../assets/templates/page.html");


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SiteState {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub website: Website,
    pub selected_project: Option<Project>,
    pub last_built: Option<DateTime<Local>>,
}

impl SiteState {
    pub fn new(title: &String, path: &PathBuf) -> SiteState {
        SiteState {
            source: path.clone(),
            website: Website::new(title),
            ..SiteState::default()
        }
    }
    pub fn get(path: &PathBuf) -> Result<SiteState, StateError> {
        let p = path.join(".site_builder");
        let f = super::get_cache_file(&p)?;
        if f.metadata()?.len() == 0 {
            let s = 
            SiteState {
                source: path.clone(),
                destination: PathBuf::default(),
                website: Website::default(),
                selected_project: None,
                last_built: None,
            };
            s.ensure_dir_defaults();
            return Ok(s)
        }
        let ret = deserialize_from(f)?;
        Ok(ret)
    }
    /// use the source property of a state instance to
    /// get the current file structure/content
    pub fn update_from_source(&mut self) -> StateResult {
        for entry in WalkDir::new(&self.source).min_depth(1).max_depth(1) {
            let entry = entry?;
            let name = entry.file_name();
            if name == "portfolio" {
                self.website.update_projects_from_source(entry.path());
            } else if name == "about.md" {
                self.website.about = super::content(entry.path());
            } else if name == "me.jpg" {
                self.website.image = entry.path().to_path_buf();
            }
        }
        Ok(String::new())
    }

    pub fn add_project(&mut self, name: String) -> StateResult {
        self.website.add_project(&self.source, name);
        match self.write_input() {
            Ok(()) => Ok(String::from("Successfully added project")),
            Err(e) => Err(StateError::new(&format!("Unable to create project, {:?}", e))),
        }
    }

    pub fn update_project(&mut self, project: Project) {
        self.website.update_project(project)
    }

    /// save a copy of the state to a cache file
    pub fn cache(&self) {
        println!("cache_state");
        if let Some(f) = self.cache_file() {
            match serialize_into(&f, self) {
                Ok(_) => (),
                Err(e) => println!("{:?}", e),
            }
        } else {
            println!("Error getting cache file")
        }
    }

    pub fn update_title(&mut self, title: String) {
        self.website.update_title(title);
    }

    /// Attempt to get the cache file. This will also
    /// ensure that the ~/.website_builder folder is created
    fn cache_file(&self) -> Option<File> {
        if let Ok(f) = super::get_cache_file(&self.source.join(".site_builder")) {
            Some(f)
        } else {
            None
        }
    }

    pub fn write_input(&self) -> ::std::io::Result<()> {
        for project in self.website.portfolio.iter() {
            let path = &project.path;
            super::ensure_folder(path)?;
            super::write_file(&mut project.description.clone(), path.join("content.md"))?;
            if let Ok(mut m) = toml::to_string(&project.meta) {
                super::write_file(&mut m, path.join("meta.toml"))?;
            }
            super::ensure_folder(&path.join("img"))?;
        }
        super::write_file(&mut self.website.about.clone(), self.source.join("about.md"))?;
        Ok(())
    }
    /// Ensure that all of the top level files and folders are
    /// included in the source dir
    pub fn ensure_dir_defaults(&self) {
        println!("Ensuring Fonts Folder");
        if let Err(e) = super::ensure_folder(&self.source.join("fonts")) {
            println!("Error ensuring folder: {:?}", e);
        }
        println!("Ensuring Portfolio Folder");
        if let Err(e) = super::ensure_folder(&self.source.join("portfolio")) {
            println!("Error ensuring folder: {:?}", e);
        }
        println!("Ensuring about.md");
        if let Err(e) = super::write_file("", self.source.join("about.md")) {
            println!("Error ensuring about.md: {:?}", e);
        }
    }
    /// Delete and recreate the output directory
    /// we don't want to have any old files laying around
    /// so we want to wipe everything first
    pub fn ensure_out_dir_defaults(&self) -> StateResult {
        super::ensure_folder(&self.destination)?;
        for entry in WalkDir::new(&self.destination).max_depth(1).min_depth(1) {
            let entry = entry?;
            if entry.file_type().is_dir() {
                remove_dir_all(&entry.path())?;
            } else {
                remove_file(&entry.path())?;
            }
        }
        super::ensure_folder(&self.destination.join("fonts"))?;
        super::ensure_folder(&self.destination.join("portfolio"))?;
        super::ensure_folder(&self.destination.join("contact"))?;
        super::ensure_folder(&self.destination.join("about"))?;
        Ok(String::from("Successfully removed and created output directories"))
    }

    pub fn add_font(&mut self, path: &PathBuf, bold: bool) -> StateResult {
        self.remove_font(bold)?;
        let msg = if bold {
            let path = super::copy_file(&path, &self.source.join("fonts"))?;
            self.website.fonts.bold = Some(path);
            "Successfully added bold font"
        } else {
            let path = super::copy_file(&path, &self.source.join("fonts"))?;
            println!("normal font: {:?}", &path);
            self.website.fonts.normal = Some(path);
            "Successfully added normal font"
        };
        Ok(msg.into())
    }

    pub fn remove_font(&mut self, bold: bool) -> StateResult {
        if bold {
            if let Some(ref p) = self.website.fonts.bold {
                if p.exists() {
                    super::remove(p).map_err(|_| format!("Error removing old font {:?}", &self.website.fonts.bold))?;
                }
            }
            self.website.fonts.bold = None;
        } else {
            if let Some(ref p) = self.website.fonts.normal {
                if p.exists() {
                    super::remove(p).map_err(|_| format!("Error removing old font {:?}", &self.website.fonts.bold))?;
                }
            }
            self.website.fonts.normal = None;
        };
        Ok(String::new())
    }
    pub fn build(&self) -> StateResult {
        let mut t = Tera::default();
        t.add_raw_templates(vec![
                            ("base.html", BASE),
                            ("about.html", ABOUT),
                            ("contact.html", CONTACT),
                            ("index.html", INDEX),
                            ("page.html", PAGE),
                            ])?;
        self.ensure_out_dir_defaults()?;
        self.move_fonts()?;
        self.build_index(&t)?;
        self.build_contact(&t)?;
        self.build_about(&t)?;
        self.build_portfolio(&t)?;
        Ok(String::from("Successfully built site!"))
    }
    fn move_fonts(&self,) -> StateResult {
        if let Some(ref normal) = self.website.fonts.normal {
            super::copy_file(normal, &self.destination.join("fonts"))?;
        }
        if let Some(ref bold) = self.website.fonts.bold {
            super::copy_file(bold, &self.destination.join("fonts"))?;
        }
        Ok(String::from("Moved fonts"))
    }
    fn build_index(&self, t: &Tera) -> StateResult {
        let mut ctx = self.get_context("index");
        let pages: Vec<IndexProject> = self.website.portfolio.iter().map(IndexProject::from).collect();
        ctx.add("pages", &pages);
        let html = t.render("index.html", &ctx)?;
        super::write_file(&html, self.destination.join("index.html"))?;
        Ok(String::from("Successfully built index"))
    }
    fn build_contact(&self, t: &Tera)  -> StateResult {
        let ctx = self.get_context("contact");
        let html = t.render("contact.html", &ctx)?;
        super::write_file(&html, self.destination.join("contact").join("index.html"))?;
        Ok(String::from("Successfully built contact"))
    }

    fn build_about(&self, t: &Tera) -> StateResult {
        let mut ctx = self.get_context("about");
        let html = super::generate_html(&self.website.about);
        ctx.add("content", &html);
        let path = super::copy_file(&self.website.image, &self.destination)?;
        let img = super::file_name(&path);
        ctx.add("image", &img);
        let body = t.render("about.html", &ctx)?;
        super::write_file(&body, self.destination.join("about").join("index.html"))?;
        Ok(String::from("Successfully Built about"))
    }

    fn build_portfolio(&self, t: &Tera) -> StateResult {
        for proj in self.website.portfolio.iter() {
            let page = Page::from(proj);
            let project_dest = self.ensure_project_folder(&page.project_folder)?;
            for img in proj.images.iter() {
                super::copy_file(&img.path, &project_dest.join("img"))?;
            }
            self.build_portfolio_page(t, &page, &project_dest)?;
        }
        Ok(String::from("Successfully Built portfolio"))
    }

    fn ensure_project_folder(&self, folder_name: &String) -> Result<PathBuf, StateError> {
        let project_path = self.destination.join("portfolio").join(folder_name);
        let img_path = project_path.join("img");
        super::ensure_folder(&img_path)?;
        Ok(project_path)
    }

    fn build_portfolio_page(&self, t: &Tera, page: &Page, dest: &PathBuf) -> StateResult {
        let mut ctx = self.get_context("portfolio");
        ctx.add("page", &page);
        let html = t.render("page.html", &ctx)?;
        super::write_file(&html, dest.join("index.html"))?;
        Ok(String::from("Successfully built project"))
    }

    fn get_context(&self, route: &str) -> Context {
        let mut ctx = Context::new();
        ctx.add("route", route);
        ctx.add("bold_font", &self.website.fonts.bold_file());
        ctx.add("normal_font", &self.website.fonts.normal_file());
        ctx.add("title", &self.website.title);
        ctx
    }

    pub fn selected_project(&mut self) -> Result<&mut Project, StateError> {
        if let Some(ref mut p) = self.selected_project {
            Ok(p)
        } else {
            Err(StateError::new("No project selected"))
        }
    }

    pub fn add_project_image(&mut self, path: &PathBuf) -> Result<(), StateError> {
        let proj = self.selected_project()?;
        proj.add_image(path)
    }
}