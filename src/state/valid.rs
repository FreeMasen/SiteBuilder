use error::{StateError, StateResult};

use fonts::Fonts;
use image::Image;
use meta::Meta;
use project::Project;
use state::SiteState;
use website::Website;

pub trait Valid {
    fn is_valid(&self) -> StateResult;
}

impl Valid for SiteState {
    fn is_valid(&self) -> StateResult {
        if !self.source.exists() {
            return Err(StateError::new("Source is required"))
        }
        if !self.destination.exists() {
            return Err(StateError::new("Destination is required"))
        }
        self.website.is_valid()?;
        Ok(String::from("AppState is Valid"))
    }
}
impl Valid for Project {
    fn is_valid(&self) -> StateResult {
        if !self.path.exists() {
            return Err(StateError::new(&format!("Path for {} was not found", &self.meta.title)));
        }
        for i in self.images.iter() {
            i.is_valid()?;
        }
        Ok(String::from("Project is Valid"))
    }
}
impl Valid for Meta {
    fn is_valid(&self) -> StateResult {
        if self.title.len() < 1 {
            return Err(StateError::new("Found empty project title"));
        }
        if self.context.len() < 1 {
            return Err(StateError::new(&format!("{} has an empty project subtitle", &self.context)));
        }
        Ok(String::from("Meta is valid"))
    }
}

impl Valid for Image {
    fn is_valid(&self) -> StateResult {
        if !self.path.exists() {
            return Err(StateError::new(&format!("Image was not found at {:?}", &self.path)));
        }
        Ok(String::new())
    }
}

impl Valid for Fonts {
    fn is_valid(&self) -> StateResult {
        if let Some(ref n) = self.normal {
            if !n.exists() {
                return Err(StateError::new("Normal font was not found"));
            }
        } else {
            return Err(StateError::new("Normal font was not found"));
        }
        if let Some(ref b) = self.bold {
            if !b.exists() {
                return Err(StateError::new("Bold font was not found"));
            }
        } else {
            return Err(StateError::new("Normal font was not found"));
        }
        Ok(String::new())
    }
}

impl Valid for Website {
    fn is_valid(&self) -> StateResult {
        if self.about.len() < 1 {
            return Err(StateError::new("About cannot be empty"))
        }
        if !self.image.exists() {
            return Err(StateError::new("Image for about page is required"))
        }
        for e in self.portfolio.iter() {
            e.is_valid()?;
        }
        self.fonts.is_valid()?;
        Ok(String::from("Website is valid"))
    }
}