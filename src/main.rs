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
use fs::*;

const INDEX: &'static str = include_str!("assets/index.html");
const JS: &'static str = include_str!("assets/app.js");
const CSS: &'static str = include_str!("assets/main.css");

fn main() {
    println!("starting");
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
                    cache_and_inject(wv, &state);
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
                    cache_and_inject(wv, &state);
                },
                //When the app requests a new project
                //be added to the portfolio we do that
                //and update the file system
                Message::AddProject {name} => {
                    println!("Add: {}", name);
                    match state.add_project(name) {
                        Ok(()) => cache_and_inject(wv, &state),
                        Err(e) => println!("Add Error: {:?}", e),
                    }
                },
                //When the app requests an update to a project
                //we find that projected and replace it with the
                //app's version and  re-write the folder/files
                Message::UpdateProject {project} => {
                    state.website.update_project(project);
                    match write_input(state) {
                        Ok(()) => cache_and_inject(wv, &state),
                        Err(e) => {
                            println!("Update Error: {:?}", e);
                        }
                    }
                },
                //When the app requests to update the about page's
                //content, we update the file system
                Message::UpdateAbout { content } => {
                    println!("UpdateAbout: {:?}", content);
                    let old_about = state.website.about.clone();
                    state.website.about = content;
                    match write_input(state) {
                        Ok(()) => cache_and_inject(wv, &state),
                        Err(e) => {
                            println!("Error: {:?}", e);
                            state.website.about = old_about;
                        }
                    }
                },
                Message::UpdateAboutImage => {
                    if let Some(p) = open_dialog(state.source.to_str(), false) {
                        let path = PathBuf::from(p);
                        match copy_file(&path, &state.source) {
                            Ok(path) => {
                                state.website.image = path;
                                cache_and_inject(wv, &state);
                            },
                            Err(e) => println!("Error moving me image {:?}", e),
                        }
                    }
                }
                //When the app wants to log some info
                Message::Log { msg } => println!("Log: {}", msg),
                Message::AddProjectImage { name } => {
                    if let Some(p) = open_dialog(state.source.to_str(), false) {
                        let source = PathBuf::from(p);
                        match copy_file(&source, &state.source.join("portfolio").join(name).join("img")) {
                            Ok(_path) => {
                                state.update_from_source();
                                cache_and_inject(wv, state);
                            },
                            Err(e) => println!("Error moving project image {:?}", e),
                        }
                    }
                },
                Message::RemoveProjectImage { path } => {
                    match remove(&path) {
                        Ok(()) => {
                            state.update_from_source();
                            cache_and_inject(wv, state)
                        },
                        Err(e) => println!("Error removing file {:?}", e),
                    }
                },
                Message::UpdateSource => {
                    if let Some(p) = open_dialog(state.source.to_str(), true) {
                        let path = PathBuf::from(p);
                        ensure_dir_defaults(&path);
                        state.source = path;
                        state.update_from_source();
                        cache_and_inject(wv, state);
                    }
                },
                Message::UpdateDest => {
                    if let Some(p) = open_dialog(state.source.to_str(), true) {
                        state.destination = PathBuf::from(p);
                        cache_and_inject(wv, state);
                    }
                },
                Message::ChangeView { route, project } => {
                    state.current_view = route;
                    state.selected_project = project;
                    cache_and_inject(wv, &state);
                },
                Message::AddFont { bold } => {
                    if let Some(p) = open_dialog(state.source.to_str(), false) {
                        let path = PathBuf::from(p);
                        match copy_file(&path, &state.source) {
                            Ok(p) => {
                                if bold {
                                    state.website.fonts.bold = Some(p);
                                } else {
                                    state.website.fonts.normal = Some(p);
                                }
                                cache_and_inject(wv, state)
                            },
                            Err(e) => println!("Error adding font {:?}", e),
                        }
                    }
                },
                Message::RemoveFont { bold } => {
                   let path = if bold {
                       state.website.fonts.bold.clone()
                   } else {
                       state.website.fonts.normal.clone()
                   };
                    if let Some(ref path) = path {
                        match remove(path) {
                            Ok(()) => {
                                if bold {
                                    state.website.fonts.bold = None;
                                } else {
                                    state.website.fonts.normal = None;
                                }
                                cache_and_inject(wv, &state);
                            },
                            Err(e) => println!("{:?}", e),
                        }
                    }
                },
                Message::ChangeImagePos { project_id, old_pos, new_pos } => {
                    if let Some(ref mut p) = state.website.get_project(project_id) {
                        p.update_image_position(old_pos, new_pos)
                    }
                }
            }
        },
        Err(e) => println!("Deserialize Error: {:?}", e),
    }
}

fn cache_and_inject(wv: &mut WebView<AppState>, app_state: &AppState) {
    cache_state(app_state);
    inject_event(wv, app_state);
}

fn inject_event(wv: &mut WebView<AppState>, app_state: &AppState) {
    let state_str = to_string(&app_state).unwrap_or(String::from("unable to serialize website"));
    wv.eval(&format!("window.dispatchEvent(new CustomEvent('state-change', {{detail: {}}}));", state_str));
}

fn open_dialog(path: Option<&str>, dir: bool) -> Option<String> {
    if dir {
        picker_result(open_pick_folder(path))
    } else {
        picker_result(open_file_dialog(path, None))
    }
}

fn picker_result(res: nfd::Result<Response>) -> Option<String> {
    match res {
        Ok(r) => {
            match r {
                Response::Okay(p) => Some(p),
                _ => None
            }
        },
        Err(e) => {
            println!("error from dialog: {:?}", e);
            None
        }
    }
}