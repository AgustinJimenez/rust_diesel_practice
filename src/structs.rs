use rocket::serde::Deserialize;
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostsCreateParams {
    pub title: String,
    pub body: String,
    pub published: bool,
}
