use askama::Template;

use super::PathPart;

#[derive(Template)]
#[template(path = "admin_dashboard.html")]
pub struct AdminDashboardTemplate<'a> {
    pub username: &'a str,
}

pub fn path() -> Vec<PathPart<'static>> {
    vec![PathPart::new("/admin/dashboard", "Dashboard")]
}
