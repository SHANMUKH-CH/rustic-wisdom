pub mod ai;
use crate::ai::chatgpt::get_chatgpt_response;

#[tokio::main]
async fn main() {
    match get_chatgpt_response("Tell me a Stoic quote.").await {
        Ok(response) => println!("Response from ChatGPT: {}", response),
        Err(e) => {
            eprintln!("Error: {}", e);
            // Add more detailed error information if possible
            if let Some(source) = e.source() {
                eprintln!("Caused by: {}", source);
            }
        },
    }
}
