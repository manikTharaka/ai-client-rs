use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use serde_json::Value;

pub struct ChatClient {
    pub token: String,
    pub url: String,
    req_client: reqwest::blocking::Client,
}

impl ChatClient {
    pub fn new(token: String) -> ChatClient {
        ChatClient {
            token: token,
            url: String::from("https://api.openai.com/v1/chat/"),
            req_client: reqwest::blocking::Client::new(),
        }
    }

    pub fn completion(
        &self,
        message: String,
        system_prompt: Option<String>,
        mdoel: Option<String>,
    ) -> String {
        let api_call: APICall = APICall {
            url: self.url.clone() + "completions",
            content_type: String::from("application/json"),
            authorization: format!("Bearer {}", self.token),
            json_body: json!({
                "model": mdoel.unwrap_or(String::from("gpt-4o")),
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
pub struct APICall {
    pub url: String,
    pub content_type: String,
    pub authorization: String,
    pub json_body: serde_json::Value,
}

pub fn get_chat_call(api_token: String, message: String, system_prompt: Option<String>) -> APICall {
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
