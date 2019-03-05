#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod database;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    database::establish_connection();
    rocket::ignite().mount("/", routes![index]).launch();
}
