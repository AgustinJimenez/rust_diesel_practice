mod models;
mod repositories;
mod routes;
mod structs;

use dotenvy::dotenv;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::{database, diesel};
use std::env;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[database("pg_db")]
pub struct DBConn(diesel::PgConnection);

#[database("pg_test_db")]
pub struct TestDBConn(diesel::PgConnection);

pub fn attach_routes(rocket_build: Rocket<Build>) -> Rocket<Build> {
    return rocket_build
        .mount("/posts", routes::posts_routes::get_routes())
        .mount("/", routes![index]);
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    attach_routes(rocket::build().attach(DBConn::fairing()))
}

pub fn rocket_test() -> Rocket<Build> {
    dotenv().ok();
    return attach_routes(rocket::build().attach(TestDBConn::fairing()));
}
