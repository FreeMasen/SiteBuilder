#[cfg(windows)]
extern crate windres;
#[cfg(windows)]
use windres::Build;
#[cfg(windows)]
use std::{env};
#[cfg(windows)]
fn main() {
    if let Ok(path) = env::current_dir() {
        Build::new().compile(path.join("site-builder.rc")).unwrap();
    }
}

#[cfg(not(windows))]
fn main() {}