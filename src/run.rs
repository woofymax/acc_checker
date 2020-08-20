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
    if let Ok(vec) = read() {
        for combo in vec {}
    }
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn read() -> Result<Vec<Vec<String>>, &'static str> {
    if let Ok(lines) = read_lines("./alts.txt") {
        print!("Ok!");
        let mut accs: Vec<Vec<String>> = Vec::new();
        for line in lines {
            if let Ok(combo) = line {
                let res: Vec<String> = combo.split(":").map(|s| s.to_string()).collect();
                accs.push(res);
            }
        }
        return Ok(accs);
    } else {
        println!(
            "{}",
            style(
                "Failed to read file. Please make sure that the file is correctly named (alts.txt)"
            )
            .red()
        );
        Err("Failed")
    }
}
async fn request(thing: Vec<Vec<String>>) -> Result<String, reqwest::Error> {
    let mut results: Vec<String> = Vec::new();

    for combo in thing {
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
    }
    deal(results);
    Ok("mmmm".to_string())
}

fn deal(json: Vec<String>) -> Result<String, String> {
    let mut v: Option<Result<String, serde_json::error::Error>> = None;
    for error in json {
        let err: &str = &error.to_owned()[..];
        v = Some(serde_json::from_str(err));
    }
    if let Some(actual_v) = v {
        match actual_v {
            Err(error) => {
                println!("{}", error);
                Err("whoops".to_string())
            }
            Ok(a) => {
                println!("{}", a);
                Ok("sike".to_string())
            }
        }
    } else {
        Err("there was an error!".to_string())
    }
}
