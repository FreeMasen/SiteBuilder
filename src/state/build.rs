use project::Project;

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct IndexProject {
    pub id: String,
    pub project_folder: String,
    pub image_name: String,
    pub title: String,
}

impl IndexProject {
    pub fn from(proj: &Project) -> IndexProject {
        let project_folder = proj.meta.title.replace(" ", "-");
        let id = String::from("project-") + &project_folder.to_lowercase();
        let image_name = match proj.images.iter().next() {
            Some(i) => super::file_name(&i.path),
            None => String::new(),
        };
        let title = proj.meta.title.clone();
        IndexProject {
            id,
            project_folder,
            image_name,
            title,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Page {
    pub title: String,
    pub sub_title: String,
    pub teammates: Vec<String>,
    pub images: Vec<String>,
    pub content: String,
    pub project_folder: String
}

impl Page {
    pub fn from(project: &Project) -> Page {
        println!("Project::from: {:?}", project.images.len());
        let title = project.meta.title.clone();
        let sub_title = project.meta.context.clone();
        let teammates = project.meta.teammates.clone();
        let images = project.images.iter().map(|i| {
            println!("mapping image: {:?}", &i);
            super::file_name(&i.path)
        }).collect();
        let content = super::generate_html(&project.description);
        let project_folder = project.meta.title.replace(" ", "-");
        Page {
            title,
            sub_title,
            teammates,
            images,
            content,
            project_folder
        }
    }
}