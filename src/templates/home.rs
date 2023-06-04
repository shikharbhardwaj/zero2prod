use askama::Template;

use super::PathPart;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate;

pub fn path() -> Vec<PathPart<'static>> {
    vec![PathPart::new("/", "Home")]
}
