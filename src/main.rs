use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use dotenv::dotenv;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    println!("🐞 Rust AI Debugger CLI");

    let api_key = env::var("GEMINI_API_KEY")
        .expect("API key not found in .env");

    println!("Enter file path to debug:");
    let mut path = String::new();
    io::stdin().read_line(&mut path)?;
    let path = path.trim();

    let code = fs::read_to_string(path)?;

    let prompt = format!(
        "Analyze this code and identify bugs, issues, and improvements:\n\n{}",
        code
    );

    let client = Client::new();

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent?key={}",
        api_key
    );

    let body = json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }]
    });

    let response = client
        .post(&url)
        .json(&body)
        .send()?
        .json::<serde_json::Value>()?;

    let reply = response["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("No response");

    println!("\n🧠 Debug Report:\n{}", reply);

    Ok(())
}