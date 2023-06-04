use askama::Template;

use super::{admin_dashboard, PathPart};

#[derive(Template)]
#[template(path = "change_password.html")]
pub struct ChangePasswordTemplate<'a> {
    pub error: &'a str,
    pub info: &'a str,
}

pub fn path() -> Vec<PathPart<'static>> {
    let mut path = admin_dashboard::path();
    path.push(PathPart::new("/admin/password", "Change password"));
    path
}
