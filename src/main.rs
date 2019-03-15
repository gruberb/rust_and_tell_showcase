#![feature(async_await, futures_api)]

pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate serde_urlencoded;

use serde::{Serialize, Deserialize};
use std::env;
use std::collections::HashMap;
use tide::{configuration::Configuration, body, head::UrlQuery};
use reqwest::StatusCode;

mod database;
mod route_handlers;


fn get_server_port() -> u16 {
    env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8181)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GitHubRedirect {
    code: String,
    state: String,
}

async fn exchange_github_token(UrlQuery(query): UrlQuery<String>) -> Result<body::Json<GitHubToken>, StatusCode> {
    println!("exchang_github_token {:?}", query);
    let query_array: GitHubRedirect = serde_urlencoded::from_str(&query).unwrap();
    println!("Huhu {:?}", query_array);
    let res = get_github_token(
        &env::var("GH_BASIC_CLIENT_ID").unwrap(), 
        &env::var("GH_BASIC_SECRET_ID").unwrap(), 
        &query_array.code, 
        &query_array.state
    );  

    let github_token: GitHubToken = serde_urlencoded::from_str(&res.unwrap()).unwrap();
    println!("answer get_github_token: {:?}", github_token);
    Ok(body::Json(github_token))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GitHubToken{
    access_token: String,
    scope: String,
    token_type: String,
}

fn get_github_token(
    client_id: &str, 
    client_secret: &str, 
    code: &str, 
    state: &str
) -> Result<String , reqwest::Error> {
    let mut params = HashMap::new();
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("code", code);
    params.insert("state", state);

    let client = reqwest::Client::new();
    let mut res = client.post("https://github.com/login/oauth/access_token")
        .form(&params)
        .send()
        .expect("Failed to send request");

    Ok(res.text()?)
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
    app.at("/callback").get(exchange_github_token);
    app.serve();
}
