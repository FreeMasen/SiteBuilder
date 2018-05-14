use std::path::PathBuf;
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub path: PathBuf,
    pub position: u32,
}