mod models;
mod repositories;
mod structs;
mod tests;

use models::post::Post;
use rocket::serde::json::Json;
use structs::PostsCreateParams;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// curl -v POST -H "Content-Type: application/json" \-d '{"title": "Some title 5", "body": "lorem ipsum 332", "published": true}' "http://127.0.0.1:8080/posts"
#[post("/", format = "application/json", data = "<params>")]
fn posts_create(params: Json<PostsCreateParams>) -> Json<Post> {
    return repositories::posts_repository::create_post(
        &params.title,
        &params.body,
        &params.published,
    );
}

// curl -v -i -X GET "http://127.0.0.1:8080/posts"
#[get("/")]
fn posts_list() -> Json<Vec<Post>> {
    return repositories::posts_repository::get_posts_json();
}

// curl -v -i -X DELETE "http://127.0.0.1:8080/posts/1"
#[delete("/<id>")]
fn post_delete(id: i32) {
    return repositories::posts_repository::delete_post(id);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/posts", routes![posts_list, post_delete, posts_create])
        .mount("/", routes![index])
}
