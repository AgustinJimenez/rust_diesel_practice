use crate::DBConn;

#[cfg(test)]
use super::rocket;
use app::establish_connection;
use app::utils::utils::get_words;
use diesel::Connection;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::serde::json::serde_json::json;

fn get_dummy_post() -> rocket::serde::json::Value {
    return json!(
        {
        "title": get_words(3..5),
        "body": get_words(5..10),
        "published": true
        }
    );
}

#[test]
fn test_create_post() {
    println!("\n\n START TEST ===>    \n\n ");
    let rocket_instance = rocket_test();
    let conn = TestDBConn::get_one(&rocket_instance).expect("database connection");
    let txn = conn
        .begin_test_transaction()
        .expect("begin test transaction");

    let client = Client::tracked(rocket_instance).expect("valid rocket instance");

    // conn.begin_test_transaction().unwrap();
    let data = get_dummy_post();

    let response = rocket::local::blocking::Client::new(&rocket)
        .post("/posts")
        .header(ContentType::JSON)
        .body(data.to_string())
        .dispatch_with(&rocket, move |req| req.data(txn.clone()));

    let post = response.into_json::<crate::Post>().unwrap();

    assert_eq!(response.status(), Status::Ok); // post was created successfully
    assert_eq!(post.title, data.get("title").unwrap().as_str().unwrap());
    assert_eq!(post.body, data.get("body").unwrap().as_str().unwrap());
    assert_eq!(
        post.published,
        data.get("published").unwrap().as_bool().unwrap()
    );

    let response = client.get("/posts").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let posts = response.into_json::<Vec<crate::Post>>().unwrap();
    let created_post = posts.into_iter().find(|item| item.id == post.id);
    assert_eq!(created_post.is_some(), true); // post was created successfully

    println!("\n\n HERE ===> response  {} \n\n ", created_post.unwrap());
}
#[test]
fn test_posts_list() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/posts").dispatch();
    let posts = response.into_json::<Vec<crate::Post>>().unwrap();
    assert_eq!(posts.len() > 0, true);
}
