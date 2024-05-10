use reqwest::header::HeaderMap;
use serde_json::Value;
use std::collections::HashMap;

async fn get(pin: &str) -> Result<HashMap<String, Value>, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("token", "".parse().unwrap());
    let url = format!("{}{}{}", "", pin, "");
    Ok(client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?)
}

async fn post() -> Result<HashMap<String, String>, reqwest::Error> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let mut data = HashMap::new();
    data.insert("user", "zhangsan");
    data.insert("password", "");

    Ok(client
        .post("https://httpbin.org/ip")
        .headers(headers)
        .json(&data)
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?)
}

#[tokio::main]
async fn main() {
    if let Ok(resp) = get("dzllikelsw").await {
        println!("{:#?}", resp);
    }

    // if let Ok(resp) = post().await {
    //     println!("{:#?}", resp);
    // }
}
