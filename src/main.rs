extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate web_view;

use serde_json::{from_str, to_string};
use web_view::{MyUnique ,WebView, Content, run};

mod state;
use state::*;


fn main() {
    let size = (800, 800);
    let resizable = true;
    let debug = false;
    
    run(
        "HW",
        Content::Html(INDEX),
        Some(size),
        resizable,
        debug,
        true,
        |_wv: MyUnique<WebView<AppState>>| {},
        event_handler,
        (),
    );
}

fn event_handler(wv: &mut WebView<AppState>, arg: &str, state: &mut AppState) {
    println!("event_loop");
    use state::ClientEvent::*;
    match from_str::<Message>(arg) {
        Ok(msg) => {
            match msg.kind {
                Init => (),
                Error => (),
                Build => (),
                Add => (),
                UpdatePage => (),
                UpdateAbout => (),
                UpdateImage => (),
                UpdateSource => (),
                UpdateDest => (),
            }
        },
        Err(e) => println!("{:?}", e),
    }
}

// fn get_state(from: PathBuf) -> AppState {

// }


fn injectEvent<T>(wv: WebView<T>, appState: State) {
    let state_str = to_string(appState);
    wv.eval(&format!("window.dispatchEvent(new CustomEvent('state-change', {{detail: }}));", state_str));
}