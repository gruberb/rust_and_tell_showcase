#![feature(async_await, futures_api)]

pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;

use serde::{Serialize, Deserialize};

use std::env;

use futures::future::FutureObj;
use tide::{configuration::Configuration, body, middleware::RequestContext, Response};
use http::status::StatusCode;

mod database;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GitHubRedirect {
    code: String,
    state: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GitHubUrl {
    base: String,
    id: String,
}

async fn get_github_url() -> Result<body::Json<GitHubUrl>, StatusCode> {
    let github_url = GitHubUrl {
        base: String::from("https://github.com/login/oauth/authorize?scope=user:email&client_id="),
        id: env::var("GH_BASIC_CLIENT_ID").unwrap(),
    };

    Ok(body::Json(github_url))
}

async fn exchange_github_token(msg: body::Json<GitHubRedirect>) -> body::Json<GitHubRedirect> {
    println!("JSON: {:?}", *msg);

    msg
}

fn get_server_port() -> u16 {
    env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8181)
}

fn debug_store(ctx: RequestContext<()>) -> FutureObj<Response> {
    println!("{:#?}", ctx.store());
    ctx.next()
}

fn main() {
    database::establish_connection();

    let mut app = tide::App::new(());
    let app_config = Configuration::build()
        .address(String::from("0.0.0.0"))
        .port(get_server_port())
        .finalize();

    app.config(app_config);
    app.middleware(debug_store);

    app.at("/login").get(get_github_url);
    app.at("/callback").get(exchange_github_token);
    app.serve();
}
