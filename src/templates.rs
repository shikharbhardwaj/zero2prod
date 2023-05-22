use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate<'a> {
    pub error: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate;
