use crate::types::Profile;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "pages/profile.html")]
pub struct ProfileTemplate {
    pub profile: Profile,
}
