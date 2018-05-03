use std::fs::{OpenOptions, File, DirBuilder};
use std::io::{Read, Write};
use std::path::{PathBuf, Path};
use std::env::var_os;

use bincode::{serialize_into, deserialize_from};
use toml;
use walkdir::{WalkDir, DirEntry, Error};

use state::{AppState, Project, Meta};

///Get the app state from our last session or default 
/// if that is unavailable
pub fn get_state() -> AppState {
    if let Some(s) = try_get_cache() {
        s
    } else {
        AppState::default()
    }
}

impl AppState {
    /// use the source property of a state instance to
    /// get the current file structure/content
    pub fn update_from_source(&mut self) {
        for entry in WalkDir::new(&self.source).min_depth(1).max_depth(1) {
            if let Ok(entry) = entry {
                let name = entry.file_name();
                if name == "portfolio" {
                    self.website.portfolio = portfolio(entry.path());
                } else if name == "about.md" {
                    self.website.about = content(entry.path());
                } else if name == "me.jpg" {
                    self.website.image = entry.path().to_path_buf();
                } else if name == "fonts" {
                    self.website.fonts = list_of_files(entry.path());
                }
            }
        }
    }

    pub fn add_project(&mut self, name: String) -> Result<(), String> {
        self.website.add_project(name);
        write_input(self)
    }
}

/// save a copy of the state to a cache file
pub fn cache_state(state: &AppState) {
    println!("cache_state");
    if let Some(f) = cache_file() {
        match serialize_into(&f, state) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    } else {
        println!("Error getting cache file")
    }
}

/// try and get our state from a previously saved
/// cache
pub fn try_get_cache() -> Option<AppState> {
   if let Some(f) = cache_file() {
        match deserialize_from(f) {
            Ok(s) => Some(s),
            Err(_e) => None
        }
   } else {
       None
   }
}

/// Attempt to get the cache file. This will also
/// ensure that the ~/.website_builder folder is created
fn cache_file() -> Option<File> {
    let db = DirBuilder::new();
    if let Some(home) = get_user_dir() {
        let path = home.join(".site_builder");
        if !path.exists() {
            if let Err(e) = db.create(&path) {
                println!("error creating cache dir {:?}", e)
            };
        }
        match OpenOptions::new().write(true).read(true).create(true).open(path.join("cache.bincode")) {
            Ok(f) => Some(f),
            Err(e) => {
                println!("error opening cache {:?}", e);
                None
            },
        }
    } else {
        None
    }
}

fn get_user_dir() -> Option<PathBuf> {
    let arg = if cfg!(windows) {
        "USERPROFILE"
    } else {
        "HOME"
    };
    if let Some(home) = var_os(arg) {
        Some(PathBuf::from(home))
    } else {
        None
    } 
}

/// Use the path to create a list of projects
fn portfolio(path: &Path) -> Vec<Project>{
    println!("portfolio({:?})", path);
    let mut ret = vec!();
    for entry in WalkDir::new(path).min_depth(1).max_depth(1) {
        if let Ok(entry) = entry {
            let mut p = project(entry.path());
            p.id = ret.len() as u32;
            ret.push(p);
        }
    }
    ret
}

/// User the path the create a single project
/// extracting the contents of content.md and meta.toml and
/// the paths from the img folder
fn project(path: &Path) -> Project {
    let mut ret = Project::default();
    for entry in WalkDir::new(path).min_depth(1).max_depth(1) {
        if let Ok(entry) = entry {
            let name = entry.file_name();
            if name == "img" {
                ret.images = list_of_files(entry.path())
            } else
            if name == "content.md" {
                ret.description = content(entry.path())
            } else
            if name == "meta.toml" {
                ret.meta = meta(entry.path())
            }
        }
    }
    ret
}

/// Walk the first level of files in this list and
/// return only files, not symlinks or directories
fn list_of_files(path: &Path) -> Vec<PathBuf> {
    WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(map_entry_to_path_buf)
        .collect()

}

/// For `filter_map`ing a WalkDir to return only the files as a path buffer
fn map_entry_to_path_buf(entry: Result<DirEntry, Error>) -> Option<PathBuf> {
    match entry {
            Ok(e) => {
                if e.file_type().is_file() {
                    Some(e.path().to_path_buf())
                } else {
                    None
                }
            },
            Err(_) => None
        }
}

/// Parse the meta.toml file for this project
fn meta(path: &Path) -> Meta {
    let ret = Meta::default();
    let mut buf = String::new();
    if let Ok(mut f) = File::open(path) {
        if let Ok(_size) = f.read_to_string(&mut buf) {
            if let Ok(m) = toml::from_str(&buf) {
                return m
            }
        }
    }
    ret
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

pub fn write_input(state: &AppState) -> Result<(), String> {
    for project in state.website.portfolio.iter() {
        let path = state.source.join("portfolio").join(&project.meta.title);
        ensure_folder(&path)?;
        write_file(&mut project.description.clone(), path.join("content.md"))?;
        if let Ok(mut m) = toml::to_string(&project.meta) {
            write_file(&mut m, path.join("meta.toml"))?;
        }
        ensure_folder(&path.join("img"))?;
    }
    write_file(&mut state.website.about.clone(), state.source.join("about.md"))?;
    Ok(())
}

pub fn ensure_folder(path: &PathBuf) -> Result<(), String> {
    if path.exists() {
        return Ok(())
    }
    let db = DirBuilder::new();
    db.create(path).map_err(|e| format!("{:?}", e))
}

pub fn write_file(content: &mut str, path: PathBuf) -> Result<(), String> {
    match File::create(&path) {
        Ok(mut f) => {
            match f.write_all(content.as_bytes()) {
                Ok(_size) => Ok(()),
                Err(e) => Err(format!("{:?}", e))
            }
        },
        Err(e) => Err(format!("{:?}", e))
    }
}

use std::collections::HashSet;
/// Ensure that all of the top level files and folders are
/// included in the source dir
pub fn ensure_dir_defaults(source: &PathBuf) {
    let files: HashSet<PathBuf> = WalkDir::new(&source).min_depth(1).max_depth(1).into_iter().filter_map(|e| {
        match e {
            Ok(e) => {
                Some(e.path().to_path_buf())
            },
            Err(_) => None
        }
    }).collect();
    let db = DirBuilder::new();
    let fonts = source.join("fonts");
    let portfolio = source.join("portfolio");
    let templates = source.join("templates");
    let about = source.join("about.md");
    let img = source.join("me.jpg");
    if files.get(&fonts).is_none() {
        let _ = db.create(&fonts);
    }
    if files.get(&portfolio).is_none() {
        let _ = db.create(&portfolio);
    } 
    if files.get(&templates).is_none() {
        let _ = db.create(&templates);
    }
    if files.get(&about).is_none() {
        let _ = File::create(&about);
    }
    if files.get(&img).is_none() {
        let _ = File::create(&img);
    }
}

pub fn copy_file(source: PathBuf, dest: &PathBuf) -> Result<(), String> {
    let mut i_f = File::open(source).map_err(map_e)?;
    let mut buf = vec!();
    i_f.read_to_end(&mut buf).map_err(map_e)?;
    let mut o_f = File::create(dest).map_err(map_e)?;
    o_f.write_all(&mut buf).map_err(map_e)?;
    Ok(())
}

fn map_e(e: ::std::io::Error) -> String {
    format!("{:?}", e)
}