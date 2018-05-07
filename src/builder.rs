use std::path::PathBuf;

use pulldown_cmark::{Parser, html::push_html};
use tera::{Tera, Context, };

use fs::{copy_file, write_file, ensure_project_folder, ensure_out_dir_defaults};
use state::{AppState, Project};

const ABOUT: &'static str = include_str!("assets/templates/about.html");
const BASE: &'static str = include_str!("assets/templates/base.html");
const CONTACT: &'static str = include_str!("assets/templates/contact.html");
const INDEX: &'static str = include_str!("assets/templates/index.html");
const PAGE: &'static str = include_str!("assets/templates/page.html");

pub fn build(state: &AppState) {
    let mut t = Tera::default();
    if let Err(e) = t.add_raw_templates(vec![
                                            ("base.html", BASE),
                                            ("about.html", ABOUT),
                                            ("contact.html", CONTACT),
                                            ("index.html", INDEX),
                                            ("page.html", PAGE),
                                            ]) {
        println!("Failed to add templates: {:?}", e);
        return;
    }
    ensure_out_dir_defaults(&state.destination);
    move_fonts(&state.website.fonts.normal, &state.website.fonts.bold, &state.destination);
    build_index(&t, &state.website.portfolio, &state.destination);
    build_contact(&t, &state.destination);
    build_about(&t, &state.website.image, &state.website.about, &state.destination);
    build_portfolio(&t, &state.website.portfolio, &state.destination);

}

fn move_fonts(normal: &Option<PathBuf>, bold: &Option<PathBuf>, dest: &PathBuf) {
    if let &Some(ref normal) = normal {
        if let Err(e) = copy_file(normal, &dest) {
            println!("Error moving normal font: {:?}", e);
        }
    }
    if let &Some(ref bold) = bold {
        if let Err(e) = copy_file(bold, &dest) {
            println!("Error moving normal font: {:?}", e);
        }
    }
}

fn build_index(t: &Tera, projects: &Vec<Project>, dest: &PathBuf) {
    let mut ctx = Context::new();
    let pages: Vec<IndexProject> = projects.iter().map(IndexProject::from).collect();
    ctx.add("pages", &pages);
    ctx.add("route", "index");
    match t.render("index.html", &ctx) {
        Ok(html) => if let Err(e) = write_file(&html, dest.join("index.html")) {
            println!("Error building index: {:?}", e);
        },
        Err(e) => println!("Error rendering index {:?}", e),
    }
}

fn build_contact(t: &Tera, dest: &PathBuf) {
    let mut ctx = Context::new();
    ctx.add("route", "contact");
    match t.render("contact.html", &ctx) {
        Ok(html) => if let Err(e) = write_file(&html, dest.join("contact").join("index.html")) {
            println!("Error writing contact file {:?}", e);
        },
        Err(e) => println!("Error building contact: {:?}", e),
    }
}

fn build_about(t: &Tera, image: &PathBuf, content: &String, dest: &PathBuf) {
    let mut ctx = Context::new();
    ctx.add("route", "about");
    let html = generate_html(&content);
    ctx.add("content", &html);
    let img = match copy_file(image, dest) {
        Ok(path) => {
            file_name(&path)
        },
        Err(e) => {
            println!("Error moving image file {:?}", e);
            String::new()
        }
    };
    ctx.add("image", &img);
    match t.render("about.html", &ctx) {
        Ok(body) => if let Err(e) = write_file(&body, dest.join("about").join("index.html")) {
            println!("Error building about: {:?}", e)
        },
        Err(e) => println!("Error building about: {:?}", e),
    }
}

fn build_portfolio(t: &Tera, projects: &Vec<Project>, dest: &PathBuf) {
    for proj in projects {
        let page = Page::from(proj);
        ensure_project_folder(&dest, &page.project_folder);
        let project_dest = dest.join("portfolio").join(&page.project_folder);
        proj.images.iter().for_each(|img| {
            if let Err(e) = copy_file(&img.path, &project_dest.join("img")) {
                println!("Error moving image: {:?}", e);
            }
        });
        build_portfolio_page(t, &page, &project_dest)
    }
}

fn build_portfolio_page(t: &Tera, page: &Page, dest: &PathBuf) {
    let mut ctx = Context::new();
    ctx.add("route", "portfolio");
    ctx.add("page", &page);
    match t.render("page.html", &ctx) {
        Ok(html) => if let Err(e) = write_file(&html, dest.join("index.html")) {
            println!("Error writing page html: {:?}", e)
        },
        Err(e) => println!("Error building page html: {:?}", e),
    }
}

fn file_name(pb: &PathBuf) -> String {
    if let Some(file_name) = pb.file_name() {
        String::from(file_name.to_string_lossy())
    } else {
        println!("Error getting file name from path: {:?}", pb);
        String::new()
    }
}

fn generate_html(md: &String) -> String {
    let p = Parser::new(md);
    let mut ret = String::new();
    push_html(&mut ret, p);
    ret
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
struct IndexProject {
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
            Some(i) => file_name(&i.path),
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
struct Page {
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
            file_name(&i.path)
        }).collect();
        let content = generate_html(&project.description);
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