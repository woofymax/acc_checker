use console::style;
use reqwest;
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;

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
    read();
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    
    Ok(io::BufReader::new(file).lines())
}
async fn read(){
    let mut accs: Vec<Vec<String>> = Vec::new();
    if let Ok(lines) = read_lines("alts.txt") {
        println!("Ok reading file");
        
        for line in lines {
            if let Ok(combo) = line {
                let res: Vec<String> = combo.split(":").map(|s| s.to_string()).collect();
                &accs.push(res);
            }
        }
    
    }
    
    
}
async fn request(creds: Vec<Vec<String>>) -> Result<&'static str, reqwest::Error> {
    let mut results: Vec<String> = Vec::new();
    println!("in the req function");
    for combo in creds {
        let echo_json = reqwest::Client::new()
            .post("https://authserver.mojang.com/authenticate")
            .json(&serde_json::json!({
                "agent": {
                    "name": "Minecraft",
                    "version": 1
                },
                "username": combo[0],
                "password": combo[1]
            }))
            .send()
            .await?
            .json()
            .await?;
        results.push(echo_json);
    }
    Ok("something")
}
