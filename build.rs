#[cfg(windows)]
extern crate windres;
#[cfg(windows)]
use windres::Build;
#[cfg(windows)]
use std::{env::var_os, path::PathBuf};
#[cfg(windows)]
fn main() {
    if let Some(arg) = var_os("APPVEYOR_BUILD_FOLDER") {
        let path = PathBuf::from(arg);
        Build::new().compile(path.join("site-builder.rc")).unwrap();
    }
}

#[cfg(not(windows))]
fn main() {}