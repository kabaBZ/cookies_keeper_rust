mod browser;
// use reqwest::cookie::CookieStore;
use reqwest::cookie;
use reqwest::redirect::Policy;
use reqwest::Result;
use serde_json;
// use std::collections::HashMap;
use serde_json::Value;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::builder()
        .cookie_provider(Arc::new(cookie::Jar::default()))
        .redirect(Policy::default())
        .build()
        .unwrap();
    let res: Value = client
        .get("http://httpbin.org/cookies/set/cookiejar/abc")
        .send()
        .await?
        .json()
        .await?;
    // let json_data: Value = serde_json::from_str(&res).unwrap();
    println!("{:#?}", res);
    Ok(())
}
