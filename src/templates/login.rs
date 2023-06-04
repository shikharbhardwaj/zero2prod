use askama::Template;

use super::{home, PathPart};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate<'a> {
    pub error: &'a str,
    pub info: &'a str,
}

pub fn path() -> Vec<PathPart<'static>> {
    let mut path = home::path();
    path.push(PathPart::new("/login", "Login"));
    path
}
