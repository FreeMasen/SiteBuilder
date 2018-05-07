use pulldown_cmark::{Parser, html::push_html};
use tera::{Tera, Context};

use state::Website;

const ABOUT: &'static str = include_str!("assets/templates/about.html");
const BASE: &'static str = include_str!("assets/templates/base.html");
const CONTACT: &'static str = include_str!("assets/templates/contact.html");
const INDEX: &'static str = include_str!("assets/templates/index.html");
const PAGE: &'static str = include_str!("assets/templates/page.html");

pub fn build(website: &Website) {
    let mut t = Tera::default();
    if let Ok(()) = t.add_raw_templates(vec![
        ("base.html", BASE),
        ("about.html", ABOUT),
        ("contact.html", CONTACT),
        ("index.html", INDEX),
        ("page.html", PAGE),
    ]) {
        
    }

}

fn generate_html(md: String) -> String {
    let p = Parser::new(&md);
    let mut ret = String::new();
    push_html(&mut ret, p);
    ret
}