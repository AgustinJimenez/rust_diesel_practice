#[cfg(test)]
mod test {
    use super::super::rocket;
    use app::utils::utils::get_words;
    use diesel::Connection;
    use rocket::local::blocking::Client;
    use rocket::http::{Status, ContentType};
    use rocket::serde::json::serde_json::json;
    use diesel::result::Error::RollbackTransaction;

    #[test]
    fn test_create_post(){
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let e_body: String = get_words(5..10);
        let e_title: String = get_words(3..5);
        let data = json!(
            {
            "title": e_title,
            "body": e_body
            }
        );

        let connection = &mut app::establish_connection();

        connection.test_transaction::<_, diesel::result::Error, _>(|conn| {

            let mut response = client
            .post("/posts")
            .header(ContentType::JSON)
            .body(data.to_string())
            .dispatch();
            assert_eq!(response.status(), Status::Ok);
            let post = response.into_json::<app::models::Post>().unwrap();
            assert_eq!(post.title, e_title);
            assert_eq!(post.body, e_body);
            assert_eq!(post.published, false);


            let mut response = client.get("/posts").dispatch();
            assert_eq!(response.status(), Status::Ok);
            let posts = response.into_json::<Vec<app::models::Post>>().unwrap();
            let created_post = posts.into_iter().find(|item| {
            let result = item.id==post.id;
            println!("\n e_id={}, id={}, title={}, body={}, published={}, result={}",post.id, item.id, item.title, item.body, item.published, result);

            result
            }).expect("Post not found");
        
            Ok(())
        }); 



 

    }
    #[test]
    fn test_posts_list() {
        
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // assert_eq!(response.into_string().unwrap(), "Hello, world!");
        // println!("\n\n HERE ===> response  {} \n\n ", rocket::serde::json::serde_json::to_string_pretty(&posts).unwrap() );
        // println!("\n\n some ===> some  {} \n\n ", posts.len() );

    }
}