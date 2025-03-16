use dotenv::dotenv;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use clap::Parser;

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
    // let api_token = api_token_result.expect("OPENAI_API_KEY must be set");

    let client = Client::new();
    
    let test_call: api::APICall = api::get_chat_call(api_token,message, None);

    
    let resp = client
        .post(test_call.url)
        .header(CONTENT_TYPE,test_call.content_type)
        .header(AUTHORIZATION, test_call.authorization)
        .json(&test_call.json_body)
        .send();

    let body = resp
        .expect("No response")
        .text()
        .expect("Failed to read response text");
    
    let body: Value = serde_json::from_str(&body).expect("Failed to parse JSON");

    if let Some(content) = body["choices"][0]["message"]["content"].as_str(){
        println!(">");
        println!("{}",content);
    }else{
        println!("No content found");
    }
}