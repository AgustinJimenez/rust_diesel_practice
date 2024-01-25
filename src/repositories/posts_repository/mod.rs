use crate::models::post::{NewPost, Post};
use crate::DBConn;
use app::schema::posts as posts_schema;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;

pub async fn get_posts(db_conn: DBConn) -> Vec<Post> {
    let results = db_conn
        .run(|c| {
            posts_schema::table
                .filter(posts_schema::published.eq(true))
                .order(posts_schema::id.desc())
                .limit(10)
                .load(c)
        })
        .await
        .expect("Error loading posts");

    return results;
}

pub async fn get_posts_json(db_conn: DBConn) -> Json<Vec<Post>> {
    let result = Json(self::get_posts(db_conn).await);
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

    let post = diesel::insert_into(posts_schema::table)
        .values(&new_post)
        .get_result(connection)
        .expect("Error saving new post");

    return Json(post);
}

pub async fn delete_post(db_conn: DBConn, post_id: i32) {
    use diesel::prelude::*;

    db_conn
        .run(move |c| {
            diesel::delete(posts_schema::table.filter(posts_schema::columns::id.eq(post_id)))
                .execute(c)
        })
        .await
        .expect("Error deleting posts");
}

pub async fn get_post_by_id_json(db_conn: DBConn, post_id: i32) -> rocket::serde::json::Json<Post> {
    let post: Post = get_post_by_id(db_conn, post_id).await;
    return Json(post);
}

pub async fn get_post_by_id(db_conn: DBConn, post_id: i32) -> Post {
    let post: Post = db_conn
        .run(move |c| posts_schema::table.find(post_id).get_result::<Post>(c))
        .await
        .expect("Error loading posts");

    return post;
}
