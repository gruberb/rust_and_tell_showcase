#![feature(async_await, futures_api)]

pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate serde_urlencoded;

use std::env;

use tide::{configuration::Configuration};

mod database;
mod route_handlers;
mod auth;
mod github;

fn get_server_port() -> u16 {
    env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8181)
}

fn main() {
    database::establish_connection();

    let mut app = tide::App::new(());
    let app_config = Configuration::build()
        .address(String::from("0.0.0.0"))
        .port(get_server_port())
        .finalize();

    app.config(app_config);

    app.at("/").get(route_handlers::index);
    app.at("/callback").get(route_handlers::user_info);
    app.serve();
}
