use std::collections::HashMap;
use std::env;

pub fn get_github_token(code: &str, state: &str) -> Result<String , reqwest::Error> {
    let client_id = &env::var("GH_BASIC_CLIENT_ID").unwrap();
    let client_secret = &env::var("GH_BASIC_SECRET_ID").unwrap();
    let code = &code.to_string();
    let state = &state.to_string();
    
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