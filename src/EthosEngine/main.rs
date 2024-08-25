pub mod ai;
use crate::ai::chatgpt::get_chatgpt_response;

#[tokio::main]
async fn main() {
    let prompt = "Marcus Aurelius, Seneca and Epictetus as a software engineers, produce a small converstation among them which is insightful, thought provoking and funny.";

    match get_chatgpt_response(prompt).await {
        Ok(response) => println!("Response from Gemini-flash:\n\n{}", response),
        Err(e) => {
            eprintln!("Error: {}", e);
            if let Some(source) = e.source() {
                eprintln!("Caused by: {}", source);
            }
        },
    }
}
