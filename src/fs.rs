use std::fs::{OpenOptions, File, DirBuilder};
use std::io::{Read};
use std::path::{PathBuf, Path};

use bincode::{serialize_into, deserialize_from};
use toml;
use walkdir::WalkDir;

use state::{AppState, Website, Project, Meta};


pub fn get_state() -> AppState {
    if let Some(s) = try_get_cache() {
        update_from_source(s)
    } else {
        AppState::default()
    }
}

pub fn update_from_source(state: AppState) -> AppState {
    let mut website = Website::default();
    for entry in WalkDir::new(&state.source).min_depth(1).max_depth(1) {
        if let Ok(entry) = entry {
            let name = entry.file_name();
            if name == "portfolio" {
                website.portfolio = portfolio(entry.path());
            } else if name == "about.md" {
                website.about = content(entry.path());
            } else if name == "me.jpg" {
                website.image = entry.path().to_path_buf();
            }
        }
    }
    let s = AppState {
        website,
        ..state
    };
    cache_state(&s);
    s
}

pub fn cache_state(state: &AppState) {
    if let Some(f) = cache_file() {
        match serialize_into(&f, state) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

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

fn cache_file() -> Option<File> {
    let db = DirBuilder::new();
    let mut path = PathBuf::from("~/.website_builder");
    if let Ok(()) = db.create(&path) {
        path.push("cache.bincode");
        return match OpenOptions::new().write(true).read(true).create(true).open(path) {
            Ok(f) => Some(f),
            Err(e) => None,
        }
    }
    None
}

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

fn project(path: &Path) -> Project {
    let mut ret = Project::default();
    for entry in WalkDir::new(path).min_depth(1).max_depth(1) {
        if let Ok(entry) = entry {
            let name = entry.file_name();
            if name == "img" {
                ret.images = images(entry.path())
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

fn images(path: &Path) -> Vec<PathBuf> {
    WalkDir::new(path).min_depth(1).max_depth(1).into_iter().filter_map(|e| {
        match e {
            Ok(e) => {
                if e.file_type().is_file() {
                    Some(e.path().to_path_buf())
                } else {
                    None
                }
            },
            Err(_) => None
        }
    }).collect()
}

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