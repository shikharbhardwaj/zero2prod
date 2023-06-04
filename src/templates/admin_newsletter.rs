use askama::Template;

use super::{admin_dashboard, PathPart};

#[derive(Template)]
#[template(path = "admin_newsletter.html")]
pub struct SendNewsletterTemplate<'a> {
    pub error: &'a str,
    pub info: &'a str,
    pub idempotency_key: &'a str,
}

pub fn path() -> Vec<PathPart<'static>> {
    let mut path = admin_dashboard::path();
    path.push(PathPart::new("/admin/newsletter", "Publish newsletter"));
    path
}
