use diesel::prelude::Queryable;
use rocket::serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl fmt::Display for Post {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Post(id={}, title={}, body={}, published={})",
            self.id, self.title, self.body, self.published
        )
    }
}
