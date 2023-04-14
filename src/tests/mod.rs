#[cfg(test)]
use super::rocket;
use app::utils::utils::get_words;
use diesel::result::Error::RollbackTransaction;
use diesel::Connection;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::serde::json::serde_json::json;

#[test]
fn test_create_post() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let data = json!(
        {
        "title": get_words(3..5),
        "body": get_words(5..10),
        "published": true
        }
    );

    let connection = &mut app::establish_connection();
    connection.begin_test_transaction().unwrap();

    let mut response = client
        .post("/posts")
        .header(ContentType::JSON)
        .body(data.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok); // post was created successfully

    let post = response.into_json::<crate::Post>().unwrap();
    assert_eq!(post.title, data.get("title").unwrap().as_str().unwrap());
    assert_eq!(post.body, data.get("body").unwrap().as_str().unwrap());
    assert_eq!(
        post.published,
        data.get("published").unwrap().as_bool().unwrap()
    );

    let mut response = client.get("/posts").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let posts = response.into_json::<Vec<crate::Post>>().unwrap();
    let created_post = posts.into_iter().find(|item| item.id == post.id);
    assert_eq!(created_post.is_some(), true); // post was created successfully

    println!("\n\n HERE ===> response  {} \n\n ", created_post.unwrap());
}
#[test]
fn test_posts_list() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");

    // assert_eq!(response.into_string().unwrap(), "Hello, world!");
    // println!("\n\n HERE ===> response  {} \n\n ", rocket::serde::json::serde_json::to_string_pretty(&posts).unwrap() );
    // println!("\n\n some ===> some  {} \n\n ", posts.len() );
}
