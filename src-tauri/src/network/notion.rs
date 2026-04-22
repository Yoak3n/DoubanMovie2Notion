use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, Error,
};
use serde::Serialize;
use crate::config::Configuration;


const NOTION_API_URL: &str = "https://api.notion.com/v1/pages/";

pub async fn post_to_notion<T: Serialize + ?Sized>(body: &T) -> Result<String, Error> {
    let client = Client::new();
    let resp = client
        .post(NOTION_API_URL)
        .json(body)
        .headers(headers())
        .send().await?;
    let body = resp.text().await?;
    Ok(body)
}

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&format_bearer_token(&Configuration::global().notion.lock().unwrap().token)).unwrap(),
    );
    headers.insert(
        "Notion-Version",
        HeaderValue::from_str("2022-06-28").unwrap(),
    );
    headers
}


fn format_bearer_token(token: &str) -> String {
    let token = token.trim();
    if token.to_ascii_lowercase().starts_with("bearer ") {
        token.to_string()
    } else {
        format!("Bearer {token}")
    }
}
