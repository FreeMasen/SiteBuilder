extern crate bincode;
extern crate chrono;
extern crate nfd;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate web_view;
extern crate walkdir;

use std::path::PathBuf;
use chrono::prelude::*;
use nfd::{Response, open_pick_folder, open_file_dialog};
use serde_json::{from_str, to_string};
use web_view::{MyUnique ,WebView, Content, run};

mod state;
mod fs;
use state::{Message, AppState};
use fs::{get_state, cache_state, write_input, ensure_dir_defaults, copy_file, remove};

const INDEX: &'static str = include_str!("assets/index.html");
const JS: &'static str = include_str!("assets/app.js");
const CSS: &'static str = include_str!("assets/main.css");

fn main() {
    let size = (800, 800);
    let s = get_state();
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
                //When the initial window loads we inject the style and js
                //contents
                Message::Load => {
                    println!("Message::Load");
                    wv.inject_css(CSS);
                    wv.eval(JS);
                },
                //When the react app is initialized, we
                //send our state
                Message::Init => {
                    println!("Message::Init\n{:?}", &state);
                    inject_event(wv, state);
                },
                //When the app requests a refresh
                //we refresh the state from the file system
                Message::Refresh => {
                    state.update_from_source();
                    cache_state(state)
                }
                //When the app reports an error we
                //print it to stdout
                Message::Error {message} => {
                    println!("Client Error: {}", message)
                },
                //When the app requests a build
                //we convert the input to the output
                Message::Build => {
                    println!("Build: {:?}, {:?}", state.source, state.destination);
                    state.last_built = Some(Local::now());
                    cache_state(&state);
                    inject_event(wv, state);
                },
                //When the app requests a new project
                //be added to the portfolio we do that
                //and update the file system
                Message::Add {name} => {
                    println!("Add: {}", name);
                    match state.add_project(name) {
                        Ok(()) => {
                            cache_state(&state);
                            inject_event(wv, state);
                        },
                        Err(e) => println!("Add Error: {:?}", e),
                    }
                },
                //When the app requests an update to a project
                //we find that projected and replace it with the
                //app's version and  re-write the folder/files
                Message::UpdateProject {project} => {
                    if let Some(idx) = state.website.portfolio.iter().position(|p| p.id == project.id) {
                        let old_proj = state.website.portfolio[idx].clone();
                        state.website.portfolio[idx] = project;
                        match write_input(state) {
                            Ok(()) => {
                                cache_state(state);
                                inject_event(wv, state);
                            },
                            Err(e) => {
                                println!("Update Error: {:?}", e);
                                state.website.portfolio[idx] = old_proj;
                            }
                        }
                    }
                    
                },
                //When the app requests to update the about page's
                //content, we update the file system
                Message::UpdateAbout {image_path, content} => {
                    println!("UpdateAbout: {:?}, {:?}", image_path, content);
                    let old_about = state.website.about.clone();
                    let old_img = state.website.image.clone();
                    state.website.about = content;
                    state.website.image = image_path;
                    match write_input(state) {
                        Ok(()) => {
                            cache_state(state);
                            inject_event(wv, state);
                        },
                        Err(e) => {
                            println!("Error: {:?}", e);
                            state.website.about = old_about;
                            state.website.image = old_img;
                        }
                    }
                },
                //When the app wants to log some info
                Message::Log { msg } => println!("Log: {}", msg),
                //When the app needs a directory selection popup
                Message::OpenDialog { name } => {
                    if name == "image" {
                        if let Ok(r) = open_file_dialog(None, state.source.to_str()) {
                            if let Response::Okay(p) = r {
                                let source = PathBuf::from(p);
                                let ext = match source.clone().extension() {
                                        Some(s) => s.to_str().unwrap_or("jpg").to_owned(),
                                        None => "jpg".into()
                                    };
                                let mut dest = state.source.join("me");
                                dest.set_extension(ext);
                                match copy_file(&source, &dest) {
                                    Ok(()) => state.website.image = dest,
                                    Err(e) => println!("Error moving image {:?}", e),
                                }
                            }
                        }
                    } else if let Ok(r) = open_pick_folder(None) {
                        if let Response::Okay(p) = r {
                            if name == "source" {
                                state.source = PathBuf::from(p);
                                state.last_built = None;
                                ensure_dir_defaults(&state.source);
                            } else if name == "destination" {
                                state.destination = PathBuf::from(p);
                            }
                        } else {
                            println!("Error, response from dir select was not Okay");
                        }
                    } else {
                        println!("Error, unable to get response from open_pick_folder");
                    }
                    cache_state(state);
                    inject_event(wv, &state)
                },
                Message::ChangeView { route, project } => {
                    state.current_view = route;
                    state.selected_project = project;
                    cache_state(state);
                    inject_event(wv, &state);
                },
                Message::AddFont { path } => {
                    if let &Some(file_name) = &path.file_name() {
                        let dest = path.join(file_name);
                        match copy_file(&path, &dest) {
                            Ok(()) => {
                                state.update_from_source();
                                cache_state(state);
                                inject_event(wv, &state);
                            },
                            Err(e) => println!("{:?}", e),
                        }
                    }
                },
                Message::RemoveFont { name } => {
                    let path = state.source.join(name);
                    match remove(&path) {
                        Ok(()) => {
                            state.update_from_source();
                            cache_state(state);
                            inject_event(wv, &state);
                        },
                        Err(e) => println!("{:?}", e),
                    }
                }
            }
        },
        Err(e) => println!("Deserialize Error: {:?}", e),
    }
}


fn inject_event(wv: &mut WebView<AppState>, app_state: &AppState) {
    let state_str = to_string(&app_state).unwrap_or(String::from("unable to serialize website"));
    wv.eval(&format!("window.dispatchEvent(new CustomEvent('state-change', {{detail: {}}}));", state_str));
}