use crate::ai::chatgpt::get_chatgpt_response;
use clap::{Arg, Command};
use log::{debug, error, info};
use once_cell::sync::OnceCell;

pub mod ai;

/* TODO:
- fetch philosopher names from a database.
- add dota 2 and marvel characters.
- banter between above characters about the software engineering industry.
*/

// Initialize a OnceCell to ensure the logger is only initialized once
static LOGGER: OnceCell<()> = OnceCell::new();

#[tokio::main]
async fn main() {
    LOGGER.get_or_init(|| {
        env_logger::init();
    });

    let matches = Command::new("EthosEngine")
        .version("1.0")
        .author("TerrorBlade, shanmukhrodofatos@gmail.com")
        .about("Generates philosophical banter")
        .arg(
            Arg::new("prompt")
                .short('p')
                .long("prompt")
                .value_name("PROMPT")
                .help("Sets the prompt for the ChatGPT")
                .num_args(1)
                .required(true),
        )
        .get_matches();
    debug!(">>> {:?}", matches);
    let prompt = matches
        .get_one::<String>("prompt")
        .expect("Prompt is required");

    info!("Starting EthosEngine with prompt: {}", prompt);

    match get_chatgpt_response(prompt).await {
        Ok(response) => {
            info!("Received response from ChatGPT");
            println!("JUST FOR FUN!!:\n\n{}", response);
        }
        Err(e) => {
            error!("Error occurred: {}", e);
            if let Some(source) = e.source() {
                error!("Caused by: {}", source);
            }
            std::process::exit(1);
        }
    }
}
