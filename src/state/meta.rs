#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Meta {
    pub title: String,
    pub context: String,
    pub teammates: Vec<String>,
}
