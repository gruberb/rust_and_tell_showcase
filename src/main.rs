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
use reqwest::Client;

mod database;


#[derive(Serialize, Deserialize, Clone, Debug)]
struct GitHubUrl {
    base: String,
    id: String,
}

fn get_server_port() -> u16 {
    env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8181)
}

async fn get_github_url() -> Result<body::Json<GitHubUrl>, StatusCode> {
    let github_url = GitHubUrl {
        base: String::from("https://github.com/login/oauth/authorize?scope=user:email&state=rustandtell&client_id="),
        id: env::var("GH_BASIC_CLIENT_ID").unwrap(),
    };

    Ok(body::Json(github_url))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GitHubRedirect {
    code: String,
    state: String,
}

async fn exchange_github_token(UrlQuery(query): UrlQuery<String>) -> Result<body::Json<GitHubToken>, StatusCode> {
    let query_array: GitHubRedirect = serde_urlencoded::from_str(&query).unwrap();
    format!("{:?},{:?}", query_array.code, query_array.state);

    let res = get_github_token(
        env::var("GH_BASIC_CLIENT_ID").unwrap(), 
        env::var("GH_BASIC_CLIENT_SECRET").unwrap(), 
        query_array.code, 
        "google.com".to_string(), 
        query_array.state);

    let get_github_token: GitHubToken = res.unwrap();

    Ok(body::Json(get_github_token))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GitHubToken{
    access_token: String,
    scope: String,
    token_type: String,
}

fn get_github_token(
    client_id: String, 
    client_secret: String, 
    code: String, 
    redirect_uri: String, 
    state: String
) -> Result<GitHubToken , reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("clientId", client_id);
    map.insert("client_secret", client_secret);
    map.insert("code", code);
    map.insert("redirect_uri", redirect_uri);
    map.insert("state", state);

    let client = reqwest::Client::new();
    let mut res = client.post("https://github.com/login/oauth/access_token")
        .json(&map)
        .send()?;

    Ok(res.json()?)
}

fn main() {
    database::establish_connection();

    let mut app = tide::App::new(());
    let app_config = Configuration::build()
        .address(String::from("0.0.0.0"))
        .port(get_server_port())
        .finalize();

    app.config(app_config);

    app.at("/login").get(get_github_url);
    app.at("/callback").get(exchange_github_token);
    app.serve();
}
