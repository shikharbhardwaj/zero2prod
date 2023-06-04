pub mod admin_dashboard;
pub mod admin_newsletter;
pub mod change_password;
pub mod home;
pub mod login;
pub mod signup;

pub use admin_dashboard::*;
pub use admin_newsletter::*;
pub use change_password::*;
pub use home::*;
pub use login::*;
pub use signup::*;

pub struct PathPart<'a> {
    pub link: &'a str,
    pub name: &'a str,
}

impl<'a> PathPart<'a> {
    fn new(link: &'a str, name: &'a str) -> Self {
        Self { link, name }
    }
}
