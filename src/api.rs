use serde_json::json;
pub struct APICall {
    pub url: String,
    pub content_type: String,
    pub authorization: String,
    pub json_body: serde_json::Value,
}

pub fn get_chat_call(api_token:String,message:String,system_prompt:Option<String>) -> APICall{
    APICall {
        url: String::from("https://api.openai.com/v1/chat/completions"),
        content_type: String::from("application/json"),
        authorization: format!("Bearer {}", api_token),
        json_body: json!({
            "model": "gpt-4o",
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt.unwrap_or(String::from("You are a helpful assistant."))
                },
                {
                    "role": "user",
                    "content": message
                }
            ]
        }),
    }
}


