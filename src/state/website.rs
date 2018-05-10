use std::path::{PathBuf, Path};

use walkdir::{WalkDir};

use fonts::Fonts;
use meta::Meta;
use project::Project;
use error::{StateError, StateResult};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Website {
    pub title: String,
    pub portfolio: Vec<Project>,
    pub about: String,
    pub image: PathBuf,
    pub fonts: Fonts,
}

impl Website {
    pub fn new(title: &String) -> Website {
        Website {
            title: title.clone(),
            ..Website::default()
        }
    }
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

    pub fn update_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn update_projects_from_source(&mut self, path: &Path) {
        let mut tmp_portfolio: Vec<Project> = vec!();
        for entry in WalkDir::new(path).min_depth(1).max_depth(1) {
            if let Ok(entry) = entry {
                match self.portfolio.binary_search_by(|p| p.path().cmp(&entry.path().to_path_buf())) {
                    Ok(idx) => {
                        let mut p = self.portfolio[idx].clone();
                        p.id = tmp_portfolio.len() as u32;
                        p.path = entry.path().to_path_buf();
                        p.update_from_source();
                        tmp_portfolio.push(p);

                    },
                    Err(_) => {
                        let mut p = Project::default();
                        p.id = tmp_portfolio.len() as u32;
                        p.path = entry.path().to_path_buf();
                        p.update_from_source();
                        tmp_portfolio.push(p);
                    }
                }
            }
        }
        self.portfolio = tmp_portfolio;
    }
    pub fn delete_project(&mut self, project: &Project) -> StateResult {
            match project.delete_files() {
                Ok(()) => {
                    self.portfolio = self.portfolio.clone().into_iter().filter(|p| p.id != project.id).collect();
                    Ok("Successfully deleted proejct".into())
                },
                Err(e) => Err(StateError::new(format!("{:?}", e))),
            }
    }
}