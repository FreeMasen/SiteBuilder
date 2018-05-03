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
    pub fonts: Vec<PathBuf>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Project {
    pub id: u32,
    pub meta: Meta,
    pub images: Vec<PathBuf>,
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Meta {
    pub title: String,
    pub context: String,
    pub teammates: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum Message {
    Load,
    Init,
    Refresh,
    Error { message: String },
    Build,
    Add { name: String },
    UpdateProject { project: Project },
    UpdateAbout { image_path: PathBuf, content: String },
    Log { msg: String },
    OpenDialog { name: String },
    ChangeView { route: u32, project: Option<Project> },
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
}