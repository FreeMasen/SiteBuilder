extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate web_view;

use serde_json::{from_str, to_string};
use web_view::{MyUnique ,WebView, Content, run};

mod state;
use state::{Website, Message};

const INDEX: &'static str = include_str!("assets/index.html");
const JS: &'static str = include_str!("assets/app.js");
const CSS: &'static str = include_str!("assets/main.css");

fn main() {
    let size = (800, 800);
    let debug = true;
    let w = Website::default();
    run(
        "Site Builder",
        Content::Html(INDEX),
        Some(size),
        false,
        debug,
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
                Load => {
                    wv.inject_css(CSS);
                    wv.eval(JS);
                },
                Message::Init {source} => {
                    //TODO: parse source path for website info
                    inject_event(wv, state);
                },
                Message::Error {message} => (),
                Message::Build {source, destination} => (),
                Message::Add {name} => (),
                Message::UpdateProject { project } => (),
                Message::UpdateAbout {image_path, content} => (),
                Message::Log { msg } => println!("Log: {}", msg),
            }
        },
        Err(e) => println!("Error: {:?}", e),
    }
}

// fn get_state(from: PathBuf) -> AppState {

// }


fn inject_event(wv: &mut WebView<Website>, app_state: &Website) {
    let state_str = to_string(&app_state).unwrap_or(String::from("unable to serialize website"));
    wv.eval(&format!("window.dispatchEvent(new CustomEvent('state-change', {{detail: {}}}));", state_str));
}