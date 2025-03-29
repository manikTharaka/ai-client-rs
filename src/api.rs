use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use serde_json::Value;



pub struct ChatClient {
    pub token: String,
    pub url: String,
    req_client: reqwest::blocking::Client,
}

pub fn openai_client(api_token:String) -> ChatClient {
    let url = "https://api.openai.com/v1/chat/";
    ChatClient::new(api_token,url.to_string())
}

pub fn gemini_client(api_token:String) -> ChatClient {
    //WIP
    let url = "https://generativelanguage.googleapis.com/v1beta/";
    ChatClient::new(api_token,url.to_string())
}

impl ChatClient {
    pub fn new(token: String,url:String) -> ChatClient {
        ChatClient {
            token,
            url,
            req_client: reqwest::blocking::Client::new(),
        }
    }
    
    
    pub fn list_completions(&self) -> String {
        let api_call: APICall = APICall {
            url: self.url.clone() + "completions",
            content_type: String::from("application/json"),
            authorization: format!("Bearer {}", self.token),
            json_body: json!({ })};
        
            let resp = self
            .req_client
            .get(api_call.url)
            .header(CONTENT_TYPE, api_call.content_type)
            .header(AUTHORIZATION, api_call.authorization)
            .send();
            
            let body = resp
            .expect("No response")
            .text()
            .expect("Failed to read response text");
            
            let body: Value = serde_json::from_str(&body).expect("Failed to parse JSON");

            return body.to_string();

    }

    pub fn completion(
        &self,
        message: String,
        system_prompt: Option<String>,
        model: Option<String>,
        store: Option<bool>,
    ) -> String {
        let api_call: APICall = APICall {
            url: self.url.clone() + "completions",
            content_type: String::from("application/json"),
            authorization: format!("Bearer {}", self.token),
            json_body: json!({
                "model": model.unwrap_or(String::from("gpt-4o")),
                "messages": [
                    {
                        "role": "system",
                        "content": system_prompt.unwrap_or(String::from("You are a helpful assistant."))
                    },
                    {
                        "role": "user",
                        "content": message
                    }
                ],
                "store": store.unwrap_or(false),
            }),
        };

        let resp = self
            .req_client
            .post(api_call.url)
            .header(CONTENT_TYPE, api_call.content_type)
            .header(AUTHORIZATION, api_call.authorization)
            .json(&api_call.json_body)
            .send();

        let body = resp
            .expect("No response")
            .text()
            .expect("Failed to read response text");
        

        let body: Value = serde_json::from_str(&body).expect("Failed to parse JSON");

        if let Some(content) = body["choices"][0]["message"]["content"].as_str() {
            return content.to_string();
        } else {
            return String::from("No content found");
        }
    }
}
#[derive(Debug)]
pub struct APICall {
    pub url: String,
    pub content_type: String,
    pub authorization: String,
    pub json_body: serde_json::Value,
}
