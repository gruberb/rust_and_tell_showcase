use std::collections::HashMap;

pub fn get_github_emails(access_token: &str) -> Result<String, reqwest::Error>  {
    let mut params = HashMap::new();
    params.insert("access_token", access_token);

    let client = reqwest::Client::new();
    let mut res = client.get("https://api.github.com/user/emails")
        .form(&params)
        .send()
        .expect("Failed to send request");

    Ok(res.text()?)
}