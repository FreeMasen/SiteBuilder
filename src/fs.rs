use std::fs::{File};
use std::io::{Read};
use std::path::{PathBuf, Path};

use toml;
use walkdir::WalkDir;

use state::{Website, Project, Meta};


pub fn get_website(path: PathBuf) -> Option<Website> {
    if path == PathBuf::from("") {
        return None;
    }
    let mut ret = Website::default();
    for entry in WalkDir::new(&path).min_depth(1).max_depth(1) {
        if let Ok(entry) = entry {
            if entry.file_name() == "portfolio" {
                ret.portfolio = portfolio(entry.path());
            } else {
                println!("{:?}", entry.file_name());
            }
        }
    }
    Some(ret)
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