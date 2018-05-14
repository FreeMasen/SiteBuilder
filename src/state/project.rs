use std::{
    fs::{File, remove_dir_all},
    io::{Read},
    path::{PathBuf, Path}
};

use toml;
use walkdir::WalkDir;

use meta::Meta;
use image::Image;
use state::error::StateError;

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Project {
    pub id: u32,
    pub path: PathBuf,
    pub meta: Meta,
    pub images: Vec<Image>,
    pub description: String,
}

impl Project {
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn sort_images(&mut self) {
        self.images.sort_by(|lhs, rhs| lhs.position.cmp(&rhs.position));
        for i in 0..self.images.len() {
            self.images[i].position = i as u32;
        }
    }
}

impl Project {
    pub fn update_from_source(&mut self) {
        for entry in WalkDir::new(&self.path).min_depth(1).max_depth(1) {
            if let Ok(entry) = entry {
                let name = entry.file_name();
                println!("project file: {:?}", name);
                if name == "img" {
                    self.update_images_from_source(&entry.path());
                } else
                if name == "content.md" {
                    self.description = super::content(&entry.path());
                } else
                if name == "meta.toml" {
                    self.meta = meta(&entry.path());
                }
            }
        }
    }

    fn update_images_from_source(&mut self, path: &Path) {
        let mut tmp_images: Vec<Image> = vec!();
        for entry in WalkDir::new(path).min_depth(1).max_depth(1) {
            if let Ok(entry) = entry {
                if !entry.file_type().is_file() {
                    continue;
                }
                match self.images.iter().position(|i| i.path == entry.path().to_path_buf()) {
                    Some(idx) => {
                        let mut img = self.images[idx].clone();
                        img.path = entry.path().to_path_buf();
                        tmp_images.push(img);
                    },
                    None => {
                        let img = Image {
                            position: self.images.len() as u32,
                            path: entry.path().to_path_buf(),
                            b_w: false,
                        };
                        tmp_images.push(img);
                    }
                }
            }
        }
        self.images = tmp_images;
        self.sort_images();
    }

    pub fn delete_files(&self) -> Result<(), String> {
        match remove_dir_all(&self.path) {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("Error deleting files: {:?}", e)),
        }
    }

    pub fn add_image(&mut self, path: &PathBuf) -> Result<(), StateError> {
        super::copy_file(&path, &self.path.join("img"))?;
        let img = Image {
            position: self.images.iter().map(|i| i.position).max().unwrap_or(0),
            path: path.clone(),
            b_w: false,
        };
        println!("add_image{:?}", &super::super::serde_json::to_string(&img));

        self.images.push(img);
        Ok(())
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