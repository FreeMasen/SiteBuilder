use super::{
    get_cache_file,
    get_home,
    error::StateError,
};

use std::{
    collections::HashMap,
    path::PathBuf,
    fs::read_to_string
};

use bincode::{deserialize_from, serialize_into};

#[derive(Deserialize, Serialize,  Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub base: String,
    pub index: String,
    pub about: String,
    pub page: String,
    pub contact: String,
}

impl Template {
    pub fn update_partial(&mut self, other: PartialTemplate) {
        if let Some(base) = other.base {
            self.base = base;
        }
        if let Some(index) = other.index {
            self.index = index;
        }
        if let Some(about) = other.about {
            self.about = about;
        }
        if let Some(page) = other.page {
            self.page = page;
        }
        if let Some(contact) = other.contact {
            self.contact = contact;
        }
    }
}
#[derive(Default)]
pub struct PartialTemplate {
    pub base: Option<String>,
    pub index: Option<String>,
    pub about: Option<String>,
    pub page: Option<String>,
    pub contact: Option<String>,
}

pub fn get_templates() -> HashMap<String, Template> {
    match get_home() {
        Ok(home) => {
            match get_cache_file(&home.join(".site_builder_templates")) {
                Ok(f) => {
                    match deserialize_from(f) {
                        Ok(hm) => hm,
                        Err(e) => {
                            eprintln!("error getting cached sites: {}", e);
                            default_templates()
                        }
                    }
                },
                Err(e) => {
                    eprintln!("failed to get cached templates {:?}", e);
                    default_templates()
                },
            }
        },
        Err(e) => {
            eprintln!("failed to get home dir {:?}", e);
            default_templates()
        }
    }
}

pub fn cache_templates(templates: &HashMap<String, Template>) {
    match get_home() {
        Ok(home) => match get_cache_file(&home.join(".site_builder_templates")) {
            Ok(f) => match serialize_into(f, templates) {
                Ok(_) => println!("templates cached"),
                Err(e) => eprintln!("Error serializing into cache file: {:?}", e),
            },
            Err(e) => eprintln!("Error getting cache file {:?}", e),
        },
        Err(e) => eprintln!("Error getting home dir {:?}", e)
    }
}

pub fn partial_template_from_folder(path: &PathBuf) -> PartialTemplate {
    let mut ret = PartialTemplate::default();
    if let Ok(base) = get_html_for(path, "base.html") {
        ret.base = Some(base);
    }
    if let Ok(index) = get_html_for(path, "index.html") {
        ret.index = Some(index);
    }
    if let Ok(about) = get_html_for(path, "about.html") {
        ret.about = Some(about);
    }
    if let Ok(page) = get_html_for(path, "page.html") {
        ret.page = Some(page);
    }
    if let Ok(contact) = get_html_for(path, "contact.html") {
        ret.contact = Some(contact);
    }
    ret
}

pub fn template_from_folder(path: &PathBuf) -> Result<Template, StateError> {
    if !path.exists() {
        return Err(StateError::new("template folder path does not exists"));
    }
    Ok(Template {
        base: get_html_for(&path, "base.html")?,
        index: get_html_for(&path, "index.html")?,
        page: get_html_for(&path, "page.html")?,
        about: get_html_for(&path, "about.html")?,
        contact: get_html_for(&path, "contact.html")?,
    })
}

fn get_html_for(path: &PathBuf, file_name: &str) -> Result<String, StateError> {
    let new_path = path.join(file_name);
    if !new_path.exists() {
        return Err(
            StateError::new(
                format!(
                    "template file: {} does not exists in {}", file_name, path.display()
                    )
                )
            )
    }
    let ret = read_to_string(new_path)?;
    Ok(ret)
}

fn default_templates() -> HashMap<String, Template> {
    let mut ret = HashMap::new();
    ret.insert("Default".to_string(), Template::default());
    ret
}

impl Default for Template {
    fn default() -> Self {
        Template {
            base: BASE.to_string(),
            about: ABOUT.to_string(),
            index: INDEX.to_string(),
            page: PAGE.to_string(),
            contact: CONTACT.to_string(),
        }
    }
}

const ABOUT: &'static str = include_str!("../assets/templates/about.html");
const BASE: &'static str = include_str!("../assets/templates/base.html");
const CONTACT: &'static str = include_str!("../assets/templates/contact.html");
const INDEX: &'static str = include_str!("../assets/templates/index.html");
const PAGE: &'static str = include_str!("../assets/templates/page.html");