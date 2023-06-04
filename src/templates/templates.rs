use askama::Template;

pub struct PathPart<'a> {
    pub link: &'a str,
    pub name: &'a str,
}
