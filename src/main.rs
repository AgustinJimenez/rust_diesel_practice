use structs::PostsCreateParams;
use rocket::serde::json::Json;

mod repositories;
mod structs;
mod tests;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[post("/", format = "application/json", data = "<params>")]
fn posts_create(params: Json<PostsCreateParams>){
    repositories::posts_repository::create_post(&params.title, &params.body);
}

// curl -i -X GET "http://127.0.0.1:8080/posts"
#[get("/")]
fn posts_list()-> Json<Vec<app::models::Post>>{
    return repositories::posts_repository::get_posts_json();
}

// curl -i -X DELETE "http://127.0.0.1:8080/posts/1"
#[delete("/<id>")]
fn post_delete(id: i32){
    return repositories::posts_repository::delete_post(id);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/posts", routes![posts_list, post_delete, posts_create])
    .mount("/", routes![index])
}