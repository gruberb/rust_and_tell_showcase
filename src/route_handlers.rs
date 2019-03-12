use tide::body;
use std::env;
use reqwest::StatusCode;

pub async fn get_github_url() -> Result<body::Json<String>, StatusCode> {
    let github_base = "https://github.com/login/oauth/authorize";
    let scope = "scope=user:email";
    let state = "state=rustandtell";
    let client_id = format!("client_id={}", env::var("GH_BASIC_CLIENT_ID").unwrap());

    Ok(body::Json(format!("{}?{}&{}&{}", github_base, scope, state, client_id)))
}