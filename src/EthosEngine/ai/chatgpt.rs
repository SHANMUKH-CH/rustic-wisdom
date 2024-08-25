use std::env;
use std::error::Error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use log::{debug, error};
use env_logger;

#[derive(Serialize)]
struct ChatRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    role: String,
    parts: Vec<RequestPart>,
}

#[derive(Serialize)]
struct RequestPart {
    text: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}

// Initialize the HTTP client and request to Google API
pub async fn get_chatgpt_response(prompt: &str) -> Result<String, Box<dyn Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    // Debug print to check if the .env file is being loaded
    if let Ok(_) = env::var("GOOGLE_API_KEY") {
        debug!(".env file loaded successfully");
    } else {
        error!("Failed to load .env file");
    }

    // Debug print to check if the API key is being picked up
    let api_key = env::var("GOOGLE_API_KEY")?;
    debug!("API Key: {}", api_key);

    let client = Client::new();
    let request = ChatRequest {
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![RequestPart {
                text: prompt.to_string(),
            }],
        }],
    };

    // Print the JSON payload for debugging
    let json_payload = serde_json::to_string(&request)?;
    debug!("JSON Payload: {}", json_payload);

    let response = client
        .post("https://generativelanguage.googleapis.com/v1/models/gemini-pro:generateContent")
        .header("x-goog-api-key", api_key)
        .header("Content-Type", "application/json")
        .body(json_payload)
        .send()
        .await?;

    // Print the raw response for debugging
    let text = response.text().await?;
    debug!("Raw response: {}", text);

    // Attempt to parse the response
    let chat_response: ChatResponse = serde_json::from_str(&text)?;

    if let Some(candidate) = chat_response.candidates.get(0) {
        if let Some(part) = candidate.content.parts.get(0) {
            return Ok(part.text.clone());
        }
    }

    Err("Unexpected response format".into())
}
