#![windows_subsystem = "windows"]
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
    let size = (800, 900);
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
    let mut should_cache_and_inject = true;
    match from_str::<Message>(arg) {
        Ok(msg) => {
            match msg {
                //When the initial window loads we inject the style and js
                //contents
                Message::Load => load_event(wv),
                //When the react app is initialized, we
                //send our state
                Message::Init => inject_event(wv, state),
                //When the app requests a refresh
                //we refresh the state from the file system
                Message::Refresh => refresh_event(state),
                // When the app reports an error we
                // display it in the toast
                Message::Error {message} => error_event(state, message),
                //When the app requests a build
                //we convert the input to the output
                Message::Build => build_event(state),
                //When the app requests a new project
                //be added to the portfolio we do that
                //and update the file system
                Message::AddProject {name} => should_cache_and_inject = add_project_event(state, name),
                //When the app requests an update to a project
                //we find that projected and replace it with the
                //app's version and  re-write the folder/files
                Message::UpdateProject {project} => update_project_event(state, project),
                //When the app requests to update the about page's
                //content, we update the file system
                Message::UpdateAbout { content } => update_about_event(state, content),
                Message::UpdateAboutImage => update_about_image_event(state),
                //When the app wants to log some info
                Message::Log { msg } => {
                    println!("Log: {}", msg);
                    should_cache_and_inject = false;
                },
                Message::AddProjectImage => add_project_image_event(state),
                Message::RemoveProjectImage { path } => remove_project_image_event(state, path),
                Message::UpdateSource => update_source_event(state),
                Message::UpdateDest => should_cache_and_inject = update_dest_event(state),
                Message::ChangeView { route, project } => state.change_view(route, project),
                Message::AddFont { bold } => add_font_event(bold, state),
                Message::RemoveFont { bold } => remove_font_event(bold, state),
                Message::DeleteProject => delete_proj_event(state),
                Message::ClearMessage { id } => state.clear_message(id),
                Message::ChooseSite { idx } => choose_site_event(idx, state),
                Message::ChangeSiteTitle { title } => change_title_event(title, state),
                Message::AddSite => should_cache_and_inject = add_site_event(state),
                Message::ChangeColor { color } => change_color_event(color, state),
                Message::NewTemplate { name } => add_template_event(name, state),
                Message::UpdateTemplate { name } => update_template_event(name, state),
                Message::RemoveTemplate { name } => remove_template_event(name, state),
                Message::ExportTemplate { name } => export_template_event(name, state),
                Message::SetSiteTemplate { name } => set_site_template_event(name, state),
            }
        },
        Err(e) => println!("Deserialize Error: {:?}", e),
    };
    if should_cache_and_inject {
        cache_and_inject(wv, state)
    }
}

fn load_event(wv: &mut WebView<State>) {
    println!("Message::Load");
    wv.inject_css(CSS);
    wv.eval(JS);
}
fn refresh_event(state: &mut State) {
    match state.update_site() {
        Ok(msg) => state.add_message(msg, false),
        Err(e) => state.add_message(format!("Error refreshing {:?}", e), true),
    }
}

fn error_event(state: &mut State, message: String) {
    println!("Message::Error: {:?}", message);
    state.add_message(message, true);
}

fn build_event(state: &mut State) {
    match state.site_valid() {
        Ok(_msg) => {
            match state.build_site() {
                Ok(msg) => state.add_message(msg, false),
                Err(e) => state.add_message(e.msg, true),
            }
        },
        Err(e) => state.add_message(format!("Error building site: {}", e.msg), true),
    }
}

fn add_project_event(state: &mut State, name: String) -> bool {
    match state.add_project(name) {
        Ok(msg) => state.add_message(msg, false),
        Err(e) => state.add_message(format!("Error adding project{:?}", e), true),
    }
    true
}

fn update_project_event(state: &mut State, project: Project) {
    match state.update_project(project) {
        Ok(s) => state.add_message(s, false),
        Err(e) => state.add_message(e.msg, true),
    }
}

fn update_about_event(state: &mut State, content: String) {
    match state.update_about(content) {
        Ok(s) => state.add_message(s, false),
        Err(e) => state.add_message(e.msg, true),
    }
}

fn update_about_image_event(state: &mut State) {
    if let Some(p) = open_dialog(state.source(), false) {
        let path = PathBuf::from(p);
        match state.update_about_image(path) {
            Ok(msg) => state.add_message(msg, false),
            Err(e) => state.add_message(e.msg, true),
        }
    }
}

fn add_project_image_event(state: &mut State) {
    if let Some(p) = open_dialog(state.source(), false) {
        let p = PathBuf::from(p);
        match state.add_project_image(p) {
            Ok(msg) => state.add_message(msg, false),
            Err(e) => state.add_message(e.msg, true),
        }
    }
}

fn remove_project_image_event(state: &mut State, path: PathBuf) {
    match state.remove_project_image(path) {
        Ok(msg) => state.add_message(msg, false),
        Err(e) => state.add_message(e.msg, true)
    }
}

fn update_source_event(state: &mut State, ) {
    if let Some(p) = open_dialog(state.source(), true) {
        let p = PathBuf::from(p);
        match state.update_source(p) {
            Ok(msg) => state.add_message(msg, false),
            Err(e) => state.add_message(e.msg, true),
        }
    }
}

fn update_dest_event(state: &mut State) -> bool {
    if let Some(p) = open_dialog(state.source(), true) {
        let p = PathBuf::from(p);
        match state.update_dest(p) {
            Ok(msg) => state.add_message(msg, false),
            Err(e) => state.add_message(e.msg, true),
        }
        true
    } else {
        false
    }
}

fn add_font_event(bold: bool, state: &mut State) {
    if let Some(p) = open_dialog(state.source(), false) {
        let path = PathBuf::from(p);
        match state.add_font(&path, bold) {
            Ok(msg) => state.add_message(msg, false),
            Err(e) => state.add_message(e.msg, true),
        }
    }
}

fn remove_font_event(bold: bool, state: &mut State) {
    match state.remove_font(bold) {
        Ok(msg) => state.add_message(msg, false),
        Err(e) => state.add_message(e.msg, true),
    }
}

fn delete_proj_event(state: &mut State) {
    match state.remove_project() {
        Ok(msg) => state.add_message(msg, false),
        Err(e) => state.add_message(e.msg, true),
    }
}

fn choose_site_event(id: usize, state: &mut State) {
    match state.choose_site(id) {
        Ok(_msg) => (),
        Err(e) => state.add_message(e.msg, true),
    }
}

fn change_title_event(title: String, state: &mut State) {
    match state.update_site_title(title) {
        Ok(msg) => state.add_message(msg, false),
        Err(e) => state.add_message(e.msg, true),
    }
}

fn add_site_event(state: &mut State) -> bool {
    if let Some(p) = open_dialog(state.source(), true) {
        state.add_site(PathBuf::from(p));
        true
    } else {
        false
    }
}

fn change_color_event(color: Color, state: &mut State) {
    match state.change_color(&color) {
        Ok(msg) => state.add_message(msg, false),
        Err(e) => state.add_message(e.msg, true),
    }
}

fn add_template_event(name: String, state: &mut State) {
    if let Some(p) = open_dialog(state.source(), true) {
        let path = PathBuf::from(p);
        match state.add_new_template(name, &path) {
            Ok(msg) => state.add_message(msg, false),
            Err(e) => state.add_message(e.msg, true),
        }
    }
}

fn update_template_event(name: String, state: &mut State) {
    if let Some(p) = open_dialog(state.source(), true) {
        let path = PathBuf::from(p);
        match state.save_template_changes(name, &path) {
            Ok(msg) => state.add_message(msg, false),
            Err(e) => state.add_message(e.msg, true),
        }
    }
}

fn remove_template_event(name: String, state: &mut State) {
    match state.remove_template(name) {
        Ok(msg) => state.add_message(msg, false),
        Err(e) => state.add_message(e.msg, true),
    }
}

fn export_template_event(name: String, state: &mut State) {
    if let Some(p) = open_dialog(state.source(), true) {
        let path = PathBuf::from(p);
        match state.export_template(name, &path) {
            Ok(msg) => state.add_message(msg, false),
            Err(e) => state.add_message(e.msg, true),
        }
    }
}

fn set_site_template_event(name: String, state: &mut State) {
    match state.set_site_template(name) {
        Ok(msg) => state.add_message(msg, false),
        Err(e) => state.add_message(e.msg, true),
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