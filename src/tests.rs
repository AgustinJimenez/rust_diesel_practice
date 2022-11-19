#[cfg(test)]
mod test {
    use super::super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn test_posts_list() {
        
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/posts").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let posts = response.into_json::<Vec<app::models::Post>>().unwrap();
        // assert_eq!(response.into_string().unwrap(), "Hello, world!");
        println!("\n\n HERE ===> response  {} \n\n ", rocket::serde::json::serde_json::to_string_pretty(&posts).unwrap() );

    }
}