use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Fonts {
    pub bold: Option<PathBuf>,
    pub normal: Option<PathBuf>,
}

impl Fonts {
    pub fn normal_file(&self) -> String {
        if let Some(ref p) = self.normal {
            super::file_name(&p)
        } else {
            String::new()
        }
    }

    pub fn bold_file(&self) -> String {
        if let Some(ref p) = self.bold {
            super::file_name(&p)
        } else {
            String::new()
        }
    }
}