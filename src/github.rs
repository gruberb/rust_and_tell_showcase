pub fn get_github_emails(access_token: &str) -> Result<String, reqwest::Error>  {
    let client = reqwest::Client::new();
    let mut res = client.get("https://api.github.com/user/emails")
        .query(&[("access_token", access_token)])
        .send()
        .expect("Failed to send request");

    Ok(res.text()?)
}