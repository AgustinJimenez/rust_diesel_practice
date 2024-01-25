use rocket::{serde::json::Json, Route};

use crate::{models::post::Post, repositories, structs::PostsCreateParams, DBConn};

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
async fn posts_list(db_conn: DBConn) -> Json<Vec<Post>> {
    return repositories::posts_repository::get_posts_json(db_conn).await;
}

// curl -v -i -X DELETE "http://127.0.0.1:8080/posts/1"
#[delete("/<id>")]
async fn post_delete(db_conn: DBConn, id: i32) {
    return repositories::posts_repository::delete_post(db_conn, id).await;
}

// curl -v -i -X GET "http://127.0.0.1:8080/posts/1"
#[get("/<id>")]
async fn post_by_id(db_conn: DBConn, id: i32) -> Json<Post> {
    return repositories::posts_repository::get_post_by_id_json(db_conn, id).await;
}

pub fn get_routes() -> Vec<Route> {
    routes![posts_create, posts_list, post_delete, post_by_id]
}
