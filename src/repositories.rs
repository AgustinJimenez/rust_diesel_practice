

pub mod posts_repository{
    use diesel::{RunQueryDsl, QueryDsl};
    use rocket::serde::json::Json;

    pub fn get_posts() -> Vec<app::models::Post> {

        let connection = &mut app::establish_connection();
        let results = app::schema::posts::table
        // .filter(app::schema::posts::published::published::eq(true))
        .limit(5)
        .load::<app::models::Post>(connection)
        .expect("Error loading posts");

        return results;
    }

    pub fn get_posts_json() -> rocket::serde::json::Json<Vec<app::models::Post>> {
        let result = Json(self::get_posts());
        return result;
    }

    pub fn create_post(title: &str, body: &str) -> app::models::Post {
        let connection = &mut app::establish_connection();
        let new_post = app::models::NewPost { title, body };

        diesel::insert_into(app::schema::posts::table)
            .values(&new_post)
            .get_result(connection)
            .expect("Error saving new post")
    }

    pub fn delete_post(post_id: i32){
        use diesel::prelude::*;
        
        let connection = &mut app::establish_connection();
        let num_deleted = diesel::delete(
            app::schema::posts::table.filter(
                app::schema::posts::columns::id.eq(post_id)
            )
        )
        .execute(connection)
        .expect("Error deleting posts");
        println!("\n\nDELETE ===> id = {}, num deleted = {} \n\n", post_id, num_deleted)
    }

}
