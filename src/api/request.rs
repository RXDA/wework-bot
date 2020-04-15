// use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use reqwest::Result as ReqwestResult;
// use std::collections::HashMap;

extern crate reqwest;


const HOOK_URL: &str =
    "https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=60738baf-3086-4f7a-94d0-f5606cd766e6";

#[derive(Serialize, Deserialize)]
pub struct Text {
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleMessage {
    #[serde(rename = "msgtype")]
    pub msg_type: String,
    pub text: Text,
}

pub fn request(sm: SimpleMessage) -> String {
    println!("{}", HOOK_URL);
    let names = vec!["Bob", "Frank", "Ferris"];
    let j = serde_json::to_string(&sm).unwrap();
    // println!("{}", j);
    let result = request_api(j);
    result.unwrap()
}

fn request_api(s: String) -> ReqwestResult<String> {
    let client = reqwest::blocking::Client::new();
    let body = client.post("http://httpbin.org/post")
        .body(s)
        .send()?.text()?;
    Ok(body)
}