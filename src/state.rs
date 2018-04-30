use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppState {
    pub portfolio: Vec<Page>,
    pub about: String,
    pub image: PathBuf,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub meta: Meta,
    pub project: Project,
    pub content: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub title: String,
    pub context: String,
    pub teammates: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub images: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    pub kind: ClientEvent,
    pub data: String
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientEvent {
    Init,
    Error,
    Build,
    Add,
    UpdatePage,
    UpdateAbout,
    UpdateImage,
    UpdateSource,
    UpdateDest,
}