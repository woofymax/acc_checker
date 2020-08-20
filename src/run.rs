use console::style;
use reqwest;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use serde_json;
pub async fn init() {
    if auth("asdf", "asdf") && proxy_connect("abc", "prxy") {
        start().await;
    }
}

fn auth(user: &str, pass: &str) -> bool {
    if user != "" && pass != "" {
        return true;
    } else {
        return true;
    }
}

fn proxy_connect(ip: &str, port: &str) -> bool {
    if ip != "" && port != "" {
        return true;
    } else {
        return true;
    }
}
async fn start() -> Result<(), reqwest::Error> {
    println!("\n\n{}\n\n", style("Started Checking").red().bold());
    let echo_json: serde_json::Value = reqwest::Client::new()
        .post("https://authserver.mojang.com/authenticate")
        .json(&serde_json::json!({
            "agent": {
                "name": "Minecraft",
                "version": 1
            },
            "username": "smth",
            "password": "smth"
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", echo_json);
    Ok(())
}
