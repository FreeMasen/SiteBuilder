extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate web_view;
extern crate walkdir;
extern crate nfd;

use nfd::{Response, open_pick_folder};
use serde_json::{from_str, to_string};
use web_view::{MyUnique ,WebView, Content, run, Dialog};

mod state;
mod fs;
use state::{Website, Message};
use fs::get_website;

const INDEX: &'static str = include_str!("assets/index.html");
const JS: &'static str = include_str!("assets/app.js");
const CSS: &'static str = include_str!("assets/main.css");

fn main() {
    let size = (800, 800);
    let w = Website::default();
    run(
        "Site Builder",
        Content::Html(INDEX),
        Some(size),
        true,
        true,
        true,
        |_wv: MyUnique<WebView<Website>>| {},
        event_handler,
        w,
    );
}

fn event_handler(wv: &mut WebView<Website>, arg: &str, state: &mut Website) {
    println!("event_loop {}", arg);
    match from_str::<Message>(arg) {
        Ok(msg) => {
            match msg {
                Message::Load => {
                    println!("Message::Load");
                    wv.inject_css(CSS);
                    println!("eval: {}", wv.eval(JS));
                },
                Message::Init {source} => {
                    //TODO: parse source path for website info
                    println!("Message::Init {:?}", source);
                    if let Some(ws) = get_website(source) {
                        inject_event(wv, &ws);
                    } else {
                        inject_event(wv, state);
                    }
                },
                Message::Error {message} => {
                    println!("Error: {}", message)
                },
                Message::Build {source, destination} => {
                    println!("Build: {:?}, {:?}", source, destination)
                },
                Message::Add {name} => {
                    println!("Add: {}", name)
                },
                Message::UpdateProject {project} => println!("UpdateProject: {:?}", project),
                Message::UpdateAbout {image_path, content} => println!("UpdateAbout: {:?}, {:?}", image_path, content),
                Message::Log { msg } => println!("Log: {}", msg),
                Message::OpenDialog { name } => {
                    if let Ok(r) = open_pick_folder(None) {
                        match r {
                            Response::Okay(p) => println!("single file {}", p),
                            Response::OkayMultiple(ps) => println!("multiples {:?}", ps),
                            Response::Cancel => println!("Cancel"),
                        }
                    }
                }
            }
        },
        Err(e) => println!("Error: {:?}", e),
    }
}


fn inject_event(wv: &mut WebView<Website>, app_state: &Website) {
    let state_str = to_string(&app_state).unwrap_or(String::from("unable to serialize website"));
    wv.eval(&format!("window.dispatchEvent(new CustomEvent('state-change', {{detail: {}}}));", state_str));
}