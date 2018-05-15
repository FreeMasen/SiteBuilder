#![allow(unknown_lints)]
#![warn(clippy)]
extern crate bincode;
extern crate chrono;
extern crate nfd;
extern crate pulldown_cmark;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tera;
extern crate toml;
extern crate web_view;
extern crate walkdir;

use std::{
    path::{PathBuf},
    process::{exit},
};
use nfd::{Response, open_pick_folder, open_file_dialog};
use serde_json::{from_str, to_string};
use web_view::{MyUnique ,WebView, Content, run};

mod messaging;
mod state;
use messaging::Message;
use state::*;

const INDEX: &'static str = include_str!("assets/index.html");
const JS: &'static str = include_str!("assets/app.js");
const CSS: &'static str = include_str!("assets/main.css");

fn main() {
    println!("starting");
    let size = (800, 800);
    let s = match State::get() {
        Ok(m) => m,
        Err(e) => {
            println!("exiting: {:?}", e);
            exit(1);
        },
    };
    run(
        "Site Builder",
        Content::Html(INDEX),
        Some(size),
        true,
        true,
        true,
        |_wv: MyUnique<WebView<State>>| {},
        event_handler,
        s,
    );
}

fn event_handler(wv: &mut WebView<State>, arg: &str, state: &mut State) {
    println!("event_handler {:?}", arg);
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
                    inject_event(wv, state);
                },
                //When the app requests a refresh
                //we refresh the state from the file system
                Message::Refresh => {
                    match state.update_site() {
                        Ok(msg) => state.add_message(msg, false),
                        Err(e) => state.add_message(format!("Error refreshing {:?}", e), true),
                    }
                    cache_and_inject(wv, &state);
                }
                // When the app reports an error we
                // display it in the toast
                Message::Error {message} => {
                    println!("Message::Error: {:?}", message);
                    state.add_message(message, true);
                    cache_and_inject(wv, state);
                },
                //When the app requests a build
                //we convert the input to the output
                Message::Build => {
                    match state.site_valid() {
                        Ok(_msg) => {
                            match state.build_site() {
                                Ok(msg) => state.add_message(msg, false),
                                Err(e) => state.add_message(e.msg, true),
                            }
                        },
                        Err(e) => state.add_message(format!("Error building site: {}", e.msg), true),
                    }
                    cache_and_inject(wv, &state);
                },
                //When the app requests a new project
                //be added to the portfolio we do that
                //and update the file system
                Message::AddProject {name} => {
                    match state.add_project(name) {
                        Ok(msg) => state.add_message(msg, false),
                        Err(e) => state.add_message(format!("Error adding project{:?}", e), true),
                    }
                    cache_and_inject(wv, state);
                },
                //When the app requests an update to a project
                //we find that projected and replace it with the
                //app's version and  re-write the folder/files
                Message::UpdateProject {project} => {
                    match state.update_project(project) {
                        Ok(s) => state.add_message(s, false),
                        Err(e) => state.add_message(e.msg, true),
                    }
                    cache_and_inject(wv, state);
                },
                //When the app requests to update the about page's
                //content, we update the file system
                Message::UpdateAbout { content } => {
                    match state.update_about(content) {
                        Ok(s) => state.add_message(s, false),
                        Err(e) => state.add_message(e.msg, true),
                    }
                    cache_and_inject(wv, state);
                },
                Message::UpdateAboutImage => {
                    if let Some(p) = open_dialog(state.source(), false) {
                        let path = PathBuf::from(p);
                        match state.update_about_image(path) {
                            Ok(msg) => state.add_message(msg, false),
                            Err(e) => state.add_message(e.msg, true),
                        }
                    }
                    cache_and_inject(wv, state);
                }
                //When the app wants to log some info
                Message::Log { msg } => println!("Log: {}", msg),
                Message::AddProjectImage => {
                    if let Some(p) = open_dialog(state.source(), false) {
                        let p = PathBuf::from(p);
                        match state.add_project_image(p) {
                            Ok(msg) => state.add_message(msg, false),
                            Err(e) => state.add_message(e.msg, true),
                        }
                    }
                    cache_and_inject(wv, state);
                },
                Message::RemoveProjectImage { path } => {
                    match state.remove_project_image(path) {
                        Ok(msg) => state.add_message(msg, false),
                        Err(e) => state.add_message(e.msg, true)
                    }
                    cache_and_inject(wv, state)
                },
                Message::UpdateSource => {
                    if let Some(p) = open_dialog(state.source(), true) {
                        let p = PathBuf::from(p);
                        match state.update_source(p) {
                            Ok(msg) => state.add_message(msg, false),
                            Err(e) => state.add_message(e.msg, true),
                        }
                    }
                    cache_and_inject(wv, state);
                },
                Message::UpdateDest => {
                    if let Some(p) = open_dialog(state.source(), true) {
                        let p = PathBuf::from(p);
                        match state.update_dest(p) {
                            Ok(msg) => state.add_message(msg, false),
                            Err(e) => state.add_message(e.msg, true),
                        }
                        cache_and_inject(wv, state);
                    }
                },
                Message::ChangeView { route, project } => {
                    state.change_view(route, project);
                    cache_and_inject(wv, &state);
                },
                Message::AddFont { bold } => {
                    if let Some(p) = open_dialog(state.source(), false) {
                        let path = PathBuf::from(p);
                        match state.add_font(&path, bold) {
                            Ok(msg) => state.add_message(msg, false),
                            Err(e) => state.add_message(e.msg, true),
                        }
                    }
                    cache_and_inject(wv, state);
                },
                Message::RemoveFont { bold } => {
                   match state.remove_font(bold) {
                       Ok(msg) => state.add_message(msg, false),
                       Err(e) => state.add_message(e.msg, true),
                   }
                   cache_and_inject(wv, state);
                },
                Message::DeleteProject => {
                    match state.remove_project() {
                        Ok(msg) => state.add_message(msg, false),
                        Err(e) => state.add_message(e.msg, true),
                    }
                    cache_and_inject(wv, state);
                },
                Message::ClearMessage { id } => {
                    state.clear_message(id);
                    cache_and_inject(wv, state)
                },
                Message::ChooseSite { idx } => {
                    match state.choose_site(idx) {
                        Ok(_msg) => (),
                        Err(e) => state.add_message(e.msg, true),
                    }
                    cache_and_inject(wv, state);
                }
                Message::ChangeSiteTitle { title } => {
                    match state.update_site_title(title) {
                        Ok(msg) => state.add_message(msg, false),
                        Err(e) => state.add_message(e.msg, true),
                    }
                    cache_and_inject(wv, state);
                },
                Message::AddSite => {
                    if let Some(p) = open_dialog(state.source(), true) {
                        state.add_site(PathBuf::from(p));
                        cache_and_inject(wv, state);
                    }
                },
                Message::ChangeColor { color } => {
                    match state.change_color(&color) {
                        Ok(msg) => state.add_message(msg, false),
                        Err(e) => state.add_message(e.msg, true),
                    }
                    cache_and_inject(wv, state);
                }
            }
        },
        Err(e) => println!("Deserialize Error: {:?}", e),
    }
}

fn cache_and_inject(wv: &mut WebView<State>, state: &State) {
    let _ = state.cache();
    inject_event(wv, state);
}

fn inject_event(wv: &mut WebView<State>, app_state: &State) {
    let state_str = to_string(&app_state).unwrap_or(String::from("unable to serialize website"));
    wv.eval(&format!("window.dispatchEvent(new CustomEvent('state-change', {{detail: {}}}));", state_str));
}

fn open_dialog(path: Option<&str>, dir: bool) -> Option<String> {
    if dir {
        picker_result(open_pick_folder(path))
    } else {
        picker_result(open_file_dialog(None, path))
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