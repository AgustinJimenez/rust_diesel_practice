use crate::models::new_post::NewPost;
use crate::models::post::Post;
use app::schema::posts::published;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;

pub fn get_posts() -> Vec<Post> {
    let connection = &mut app::establish_connection();
    let results = app::schema::posts::table
        .filter(app::schema::posts::published.eq(true))
        .order(app::schema::posts::id.desc())
        .limit(10)
        .load::<Post>(connection)
        .expect("Error loading posts");

    return results;
}

pub fn get_posts_json() -> rocket::serde::json::Json<Vec<Post>> {
    let result = Json(self::get_posts());
    return result;
}

pub fn create_post(
    title: &str,
    body: &str,
    is_published: &bool,
) -> rocket::serde::json::Json<Post> {
    let connection = &mut app::establish_connection();
    let new_post = NewPost {
        title,
        body,
        published: is_published,
    };

    let post = diesel::insert_into(app::schema::posts::table)
        .values(&new_post)
        .get_result(connection)
        .expect("Error saving new post");

    return Json(post);
}

pub fn delete_post(post_id: i32) {
    use diesel::prelude::*;

    let connection = &mut app::establish_connection();
    let num_deleted = diesel::delete(
        app::schema::posts::table.filter(app::schema::posts::columns::id.eq(post_id)),
    )
    .execute(connection)
    .expect("Error deleting posts");

    println!(
        "\n\nDELETE ===> id = {}, num deleted = {} \n\n",
        post_id, num_deleted
    )
}

pub fn get_post_by_id(post_id: i32) -> rocket::serde::json::Json<Post> {
    let connection = &mut app::establish_connection();
    let post: Post = app::schema::posts::table
        .find(post_id)
        .get_result(connection)
        .expect("Post not found");
    return Json(post);
    // return post;
}
