use std::path::PathBuf;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub website: Website,
    pub current_view: u32,
    pub selected_project: Option<Project>,
    pub last_built: Option<DateTime<Local>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Website {
    pub portfolio: Vec<Project>,
    pub about: String,
    pub image: PathBuf,
    pub fonts: Fonts,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Project {
    pub id: u32,
    pub path: PathBuf,
    pub meta: Meta,
    pub images: Vec<Image>,
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Fonts {
    pub bold: Option<PathBuf>,
    pub normal: Option<PathBuf>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Image {
    pub path: PathBuf,
    pub position: u32,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Meta {
    pub title: String,
    pub context: String,
    pub teammates: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum Message {
    /// Window loaded
    Load, 
    /// React app initialized
    Init, 
    // refresh state from file system
    Refresh, 
    /// report client error
    Error { message: String },
    /// build the site
    Build, 
    /// add a new project
    AddProject { name: String }, 
    /// update a project
    UpdateProject { project: Project }, 
    /// update the about section text
    UpdateAbout { content: String },
    /// update the about image
    UpdateAboutImage,
    /// Client logging
    Log { msg: String },
    /// Update the source
    UpdateSource,
    /// Update the destination
    UpdateDest,
    /// Add a project image
    AddProjectImage,
    /// remove a project image
    RemoveProjectImage { path: PathBuf},
    /// Change the current view
    ChangeView { route: u32, project: Option<Project> },
    /// Add a font file
    AddFont { bold: bool },
    /// Remove a font file
    RemoveFont { bold: bool },
    /// Delete the selected project
    DeleteProject,
}

impl Website {
    pub fn add_project(&mut self, name: String) {
        let new_project = Project {
            id: self.portfolio.len() as u32,
            meta: Meta {
                title: name,
                ..Meta::default()
            },
            ..Project::default()
        };
        self.portfolio.push(new_project);
    }

    pub fn get_project(&mut self, id: u32) -> Option<Project> {
        match  self.get_project_idx(id) {
            Some(idx) => match self.portfolio.get(idx) {
                Some(p) => Some(p.clone()),
                None => None,
            },
            None => None,
        }
    }

    pub fn update_project(&mut self, project: Project) {
        println!("update_project with id {}", &project.id);
        match self.get_project_idx(project.id) {
            Some(idx) => {
                println!("found project {}", &self.portfolio[idx].meta.title);
                self.portfolio[idx] = project
            },
            None => println!("Unable to find project with matching id"),
        }
    }

    fn get_project_idx(&self, id: u32) -> Option<usize> {
        self.portfolio.iter().position(|p| p.id == id)
    }
}

impl Project {
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn update_image_position(&mut self, old_position: u32, new_position: u32) {
        println!("old order: {:?}", &self.images);
        println!("setting {} to {}", old_position, new_position);
        self.images = self.images.clone().into_iter().map(|i| {
            if i.position == old_position {
                println!("found old position: {:?}", &i);
                Image {
                    position: new_position,
                    ..i
                }
            } else if i.position == new_position {
                Image {
                    position: old_position,
                    ..i
                }
            } else {
                i.clone()
            }
        }).collect();
        self.sort_images();
        println!("new order: {:?}",& self.images);
    }

    pub fn sort_images(&mut self) {
        self.images.sort_by(|lhs, rhs| lhs.position.cmp(&rhs.position));
        for i in 0..self.images.len() {
            self.images[i].position = i as u32;
        }
    }
}