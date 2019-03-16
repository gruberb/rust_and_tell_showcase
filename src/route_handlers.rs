use std::env;
use ramhorns::{Template, Content};
use http::Response;
use serde::{Serialize, Deserialize};
use tide::{head::UrlQuery};

use crate::auth;
use crate::github;
use crate::models;

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
    // Get the query params from  the UrlQuery String and deserialse them into
    // a GitHubRedirect struct
    let query_array: GitHubRedirect = serde_urlencoded::from_str(&query).unwrap();
    
    // Use the query params to request a GitHub token via the api
    let token = auth::get_github_token(&query_array.code, &query_array.state);
    let github_token: GitHubToken = serde_urlencoded::from_str(&token.unwrap()).unwrap();
    
    let user = models::User {
        email: String::from(""),
        token: github_token.access_token.to_owned(),
    };

    // Use the access token from the get_github_token response to fetch user information
    let result = github::get_github_emails(&github_token.access_token); 

    let result = match result {
        Ok(github_emails) => github_emails,
        Err(error) => panic!("There was a problem with getting emails: {:?}", error),

    };

    let user_emails = serde_json::from_str(&result);

    let user_emails = match user_emails {
        Ok(emails) => emails,
        Err(error) => panic!("There was a problem with parsing emails: {:?}", error)
    };

    println!("{:?}", user_emails);
    // for x in v {
    //     if x.primary {
    //         user.email = x.email;
    //     }
    // }

    // let source = "<h1>Welcome</h1>\
    //           <p>{{email}}</p>";

    // let tpl = Template::new(source).unwrap();

    // let str = tpl.render(&models::DisplayUser {
    //     email: user.email,
    // });

    Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(String::from("TEST"))
        .unwrap()
}