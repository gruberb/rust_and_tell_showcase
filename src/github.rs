use std::collections::HashMap;

pub fn get_github_emails(access_token: &str) -> reqwest::Response {
    let mut params = HashMap::new();
    params.insert("access_token", access_token);

    let client = reqwest::Client::new();
    client.post("https://api.github.com/user/emails")
        .form(&params)
        .send()
        .expect("Failed to send request")
}