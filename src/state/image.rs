use std::path::PathBuf;
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Image {
    pub path: PathBuf,
    pub position: u32,
}