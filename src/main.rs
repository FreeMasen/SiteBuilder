extern crate bincode;
extern crate nfd;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate web_view;
extern crate walkdir;

use std::path::PathBuf;

use nfd::{Response, open_pick_folder};
use serde_json::{from_str, to_string};
use web_view::{MyUnique ,WebView, Content, run};

mod state;
mod fs;
use state::{Message, AppState};
use fs::get_state;

const INDEX: &'static str = include_str!("assets/index.html");
const JS: &'static str = include_str!("assets/app.js");
const CSS: &'static str = include_str!("assets/main.css");

fn main() {
    let size = (800, 800);
    let s = AppState::default();
    run(
        "Site Builder",
        Content::Html(INDEX),
        Some(size),
        true,
        true,
        true,
        |_wv: MyUnique<WebView<AppState>>| {},
        event_handler,
        s,
    );
}

fn event_handler(wv: &mut WebView<AppState>, arg: &str, state: &mut AppState) {
    println!("event_loop {}", arg);
    match from_str::<Message>(arg) {
        Ok(msg) => {
            match msg {
                Message::Load => {
                    println!("Message::Load");
                    wv.inject_css(CSS);
                    wv.eval(JS);
                },
                Message::Init => {
                    inject_event(wv, &get_state());
                },
                Message::Error {message} => {
                    println!("Error: {}", message)
                },
                Message::Build => {
                    println!("Build: {:?}, {:?}", state.source, state.destination)
                },
                Message::Add {name} => {
                    println!("Add: {}", name)
                },
                Message::UpdateProject {project} => {
                    if let Some(idx) = state.website.portfolio.iter().position(|p| p.id == project.id) {
                        state.website.portfolio[idx] = project;
                    }
                },
                Message::UpdateAbout {image_path, content} => {
                    println!("UpdateAbout: {:?}, {:?}", image_path, content);
                    state.website.about = content;
                    state.website.image = image_path;
                },
                Message::Log { msg } => println!("Log: {}", msg),
                Message::OpenDialog { name } => {
                    if let Ok(r) = open_pick_folder(None) {
                        if let Response::Okay(p) = r {
                            if name == "source" {
                                state.source = PathBuf::from(p);
                            } else if name == "destination" {
                                state.destination = PathBuf::from(p);
                            }
                        }
                    }
                    inject_event(wv, &state)
                }
            }
        },
        Err(e) => println!("Error: {:?}", e),
    }
}


fn inject_event(wv: &mut WebView<AppState>, app_state: &AppState) {
    let state_str = to_string(&app_state).unwrap_or(String::from("unable to serialize website"));
    wv.eval(&format!("window.dispatchEvent(new CustomEvent('state-change', {{detail: {}}}));", state_str));
}