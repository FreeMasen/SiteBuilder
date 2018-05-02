use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Website {
    pub portfolio: Vec<Project>,
    pub about: String,
    pub image: PathBuf,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Project {
    pub id: u32,
    pub meta: Meta,
    pub images: Vec<PathBuf>,
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Meta {
    pub title: String,
    pub context: String,
    pub teammates: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum Message {
    Load,
    Init { source: PathBuf },
    Error { message: String },
    Build { source: PathBuf, destination: PathBuf },
    Add { name: String },
    UpdateProject { project: Project },
    UpdateAbout { image_path: PathBuf, content: String },
    Log { msg: String },
    OpenDialog { name: String },
}
