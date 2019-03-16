use std::env;
use ramhorns::{Template, Content};
use http::Response;
use serde::{Serialize, Deserialize};
use tide::{head::UrlQuery};

use crate::auth;
use crate::github;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GitHubRedirect {
    code: String,
    state: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GitHubToken{
    access_token: String,
    scope: String,
    token_type: String,
}

#[derive(Content)]
struct Login {
    title: String,
    link: String,
}

#[derive(Content, Serialize, Deserialize, Clone, Debug)]
struct UserInfo {
    emails: Vec<String>,
}

pub async fn index() -> Response<String> {
    let github_base = "https://github.com/login/oauth/authorize";
    let scope = "scope=user:email";
    let state = "state=rustandtell";
    let client_id = format!("client_id={}", env::var("GH_BASIC_CLIENT_ID").unwrap());

    let source = "<h1>Welcome to Rust and Tell Berlin</h1>\
                <button onclick=location.href='{{link}}' type='button'>
                    {{title}}
                </button>";

    let tpl = Template::new(source).unwrap();

    let str = tpl.render(&Login {
        title: "Login".to_string(),
        link: format!("{}?{}&{}&{}", github_base, scope, state, client_id),
    });

    Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(str)
        .unwrap()
}

pub async fn user_info(UrlQuery(query): UrlQuery<String>) -> Response<String> {
    let query_array: GitHubRedirect = serde_urlencoded::from_str(&query).unwrap();
    
    let token = auth::get_github_token(&query_array.code, &query_array.state);
    let github_token: GitHubToken = serde_urlencoded::from_str(&token.unwrap()).unwrap();
    
    let emails = github::get_github_emails(&github_token.access_token);
    let user_info: UserInfo = serde_urlencoded::from_str(&emails.unwrap()).unwrap();

    let source = "<h1>Welcome</h1>\
              {{#emails}}<p>{{email}}</p>{{/emails}}";

    let tpl = Template::new(source).unwrap();

    let str = tpl.render(&UserInfo {
        emails: user_info.emails,
    });

    Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(str)
        .unwrap()
}