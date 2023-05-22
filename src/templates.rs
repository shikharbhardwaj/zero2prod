use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate<'a> {
    pub error: &'a str,
    pub info: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate;

#[derive(Template)]
#[template(path = "admin_dashboard.html")]
pub struct AdminDashboardTemplate<'a> {
    pub username: &'a str,
}

#[derive(Template)]
#[template(path = "change_password.html")]
pub struct ChangePasswordTemplate<'a> {
    pub error: &'a str,
    pub info: &'a str,
}
