pub mod admin_dashboard;
pub mod admin_newsletter;
pub mod change_password;
pub mod home;
pub mod login;
pub mod signup;

pub use admin_dashboard::AdminDashboardTemplate;
pub use admin_newsletter::SendNewsletterTemplate;
pub use change_password::ChangePasswordTemplate;
pub use home::HomeTemplate;
pub use login::LoginTemplate;
pub use signup::SignupTemplate;

pub struct PathPart<'a> {
    pub link: &'a str,
    pub name: &'a str,
}

impl<'a> PathPart<'a> {
    fn new(link: &'a str, name: &'a str) -> Self {
        Self { link, name }
    }
}
