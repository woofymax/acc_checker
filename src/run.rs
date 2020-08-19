use console::style;
use std::collections::HashMap;
use json;
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

async fn start() {
    println!("\n\n{}\n\n", style("Started Cracking").red().bold());
    let map = "";
    let client = reqwest::Client::new();
    let res = client
        .post("authserver.mojang.com/authenticate")
        .json(&map)
        .send()
        .await;

    match res {
        Ok(r) => println!("HELP {:?}", r),
        Err(e) => println!("{}", e),
    }
}
