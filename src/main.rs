use dotenv::dotenv;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use clap::Parser;
use api::Chat;
mod api;



#[derive(Parser)]
#[command(name = "llm-client")]
struct Args{
    message: String,
    system_prompt: Option<String>,
}


fn main() {
    let args = Args::parse();
    let message = args.message;

    println!("Message: {}",message);

    dotenv().ok();
    let api_token = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    let chat_client = api::OpenAIClient::new(api_token, None, None);

    println!("{}", chat_client.chat(message));
    
}