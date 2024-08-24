use std::env;
use std::error::Error;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Option<Vec<Choice>>,
    error: Option<ApiError>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

#[derive(Deserialize)]
struct ApiError {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
}

// Initialize the HTTP client and request to ChatGPT API
pub async fn get_chatgpt_response(prompt: &str) -> Result<String, Box<dyn Error>> {
    dotenv::dotenv().ok(); // Load environment variables

    // Debug print to check if the .env file is being loaded
    if let Ok(_) = env::var("ANTHROPIC_API_KEY") {
        println!(".env file loaded successfully");
    } else {
        println!("Failed to load .env file");
    }

    // Debug print to check if the API key is being picked up
    let api_key = env::var("ANTHROPIC_API_KEY")?;
    println!("API Key: {}", api_key); // Print the API key for debugging

    let client = Client::new();
    let request_body = ChatRequest {
        model: "claude-3-5-sonnet-20240620".to_string(),
        max_tokens: 1024,
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
    };

    // Print the JSON payload for debugging
    let json_payload = serde_json::to_string(&request_body)?;
    println!("JSON Payload: {}", json_payload);

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .body(json_payload)
        .send()
        .await?;

    // Print the raw response for debugging
    let text = response.text().await?;
    println!("Raw response: {}", text);

    // Attempt to parse the response
    let chat_response: ChatResponse = serde_json::from_str(&text)?;

    if let Some(error) = chat_response.error {
        if error.error_type == "invalid_request_error" && error.message.contains("credit balance") {
            return Err("Your credit balance is too low to access the Claude API. Please go to Plans & Billing to upgrade or purchase credits.".into());
        }
        return Err(format!("API Error: {} - {}", error.error_type, error.message).into());
    }

    if let Some(choices) = chat_response.choices {
        if let Some(choice) = choices.get(0) {
            return Ok(choice.message.content.clone());
        }
    }

    Err("Unexpected response format".into())
}
