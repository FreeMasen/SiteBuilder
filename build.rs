#[cfg(windows)]
extern crate windres;
#[cfg(windows)]
use windres::Build;
#[cfg(windows)]
use std::{env};
#[cfg(windows)]
fn main() {
    if let Ok(path) = env::current_dir() {
        let rc_path = path.join("site-builder.rc");
        if !rc_path.exists() {
            panic!("site-builder.rs is not found at path {:?}", rc_path);
        }
        Build::new().compile(rc_path).unwrap();
    }
}

#[cfg(not(windows))]
fn main() {}