#[cfg(windows)]
extern crate windres;
#[cfg(windows)]
use windres::Build;
#[cfg(windows)]
use std::{env,
    fs::{File},
    io::{Write},
};
#[cfg(windows)]
fn main() {
    if let Ok(path) = env::current_dir() {
        let rc_path = path.join("assets").join("site-builder.rc");
        let img_path = path.join("assets").join("site-builder.ico");
        if !img_path.exists() {
            panic!("Unable to find img path: {:?}", img_path);
        }
        let content = format!("1 ICON {:?}", img_path);
        match File::create(&rc_path) {
            Ok(mut f) => {
                match f.write_all(content.as_bytes()) {
                    Ok(_size) => (),
                    Err(e) =>  panic!("Failed to write file at {:?}\n{:?}", &rc_path, e),
                }
            },
            Err(e) => panic!("Failed to create file at {:?}\n{:?}", &rc_path, e),
        }
        
        if let Err(e) = Build::new().compile(&rc_path) {
            panic!("Failed to compile {:?} with contents:\n{}\n{:?}", rc_path, content, e);
        }
    }
}

#[cfg(not(windows))]
fn main() {}