use std::env;
use ramhorns::{Template, Content};
use http::Response;

pub async fn index() -> Response<String> {
    let github_base = "https://github.com/login/oauth/authorize";
    let scope = "scope=user:email";
    let state = "state=rustandtell";
    let client_id = format!("client_id={}", env::var("GH_BASIC_CLIENT_ID").unwrap());

    #[derive(Content)]
    struct Login {
        button: String,
        link: String,
    }

    let source = "<h1>Welcome to Rust and Tell Berlin</h1>\
                <a href='{{link}}'>{{button}}</a>";

    let tpl = Template::new(source).unwrap();

    let str = tpl.render(&Login {
        button: "Login".to_string(),
        link: format!("{}?{}&{}&{}", github_base, scope, state, client_id),
    });

    Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(str)
        .unwrap()
}