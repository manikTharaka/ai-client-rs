use clap::builder::Str;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use serde_json::Value;

struct ChatClient {
    token: String,
    system_prompt: String,
    model: String,
    request: reqwest::blocking::Client,
}

pub struct OpenAIClient {
    pub url: String,
    pub store: bool,
    pub client: ChatClient,
}

pub struct GeminiClient {
    pub url: String,
    pub store: bool,
    pub client: ChatClient,
}

impl OpenAIClient {
    pub fn new(
        token: String,
        model: Option<String>,
        system_prompt: Option<String>,
    ) -> OpenAIClient {
        OpenAIClient {
            url: String::from("https://api.openai.com/v1/chat/"),
            store: false,
            client: ChatClient {
                token,
                system_prompt: system_prompt
                    .unwrap_or(String::from("You are a helpful assistant.")),
                model: model.unwrap_or(String::from("gpt-4o")),
                request: reqwest::blocking::Client::new(),
            },
        }
    }
}

impl GeminiClient{
    pub fn new(
        token: String,
        model: Option<String>,
        system_prompt: Option<String>,
    ) -> GeminiClient {
        GeminiClient {
            url: String::from("https://generativelanguage.googleapis.com/v1beta/"),
            store: false,
            client: ChatClient {
                token,
                system_prompt: system_prompt
                    .unwrap_or(String::from("You are a helpful assistant.")),
                model: model.unwrap_or(String::from("gpt-4o")),
                request: reqwest::blocking::Client::new(),
            },
        }
    }
}


pub trait Chat {
    fn chat(&self, message: String) -> String;
}

impl Chat for OpenAIClient {
    fn chat(&self, message: String) -> String {
        let api_call: APICall = APICall {
            url: self.url.clone() + "completions",
            content_type: String::from("application/json"),
            authorization: format!("Bearer {}", self.client.token),
            json_body: json!({
                "model": self.client.model,
                "messages": [
                    {
                        "role": "system",
                        "content": self.client.system_prompt
                    },
                    {
                        "role": "user",
                        "content": message
                    }
                ],
                "store": self.store,
            }),
        };

        let resp = self
            .client
            .request
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

impl Chat for GeminiClient{
    fn chat(&self, message: String) -> String {
        let api_call: APICall = APICall {
            url: self.url.clone() + "models/" + self.client.model.as_str() + ":generateContent?key=" + self.client.token.as_str(),
            content_type: String::from("application/json"),
            authorization: "".to_string(),
            json_body: json!({
                "contents": [
                    {
                      "parts": [
                        {
                          "text": message
                        }
                      ]
                    }
                  ]
            }),
        };

        let resp = self
            .client
            .request
            .post(api_call.url)
            .header(CONTENT_TYPE, api_call.content_type)
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


// impl ChatClient {
//     pub fn new(token: String,url:String) -> ChatClient {
//         ChatClient {
//             token,
//             url,
//             request: reqwest::blocking::Client::new(),
//         }
//     }

//     pub fn list_completions(&self) -> String {
//         let api_call: APICall = APICall {
//             url: self.url.clone() + "completions",
//             content_type: String::from("application/json"),
//             authorization: format!("Bearer {}", self.token),
//             json_body: json!({ })};

//             let resp = self
//             .request
//             .get(api_call.url)
//             .header(CONTENT_TYPE, api_call.content_type)
//             .header(AUTHORIZATION, api_call.authorization)
//             .send();

//             let body = resp
//             .expect("No response")
//             .text()
//             .expect("Failed to read response text");

//             let body: Value = serde_json::from_str(&body).expect("Failed to parse JSON");

//             return body.to_string();

//     }

//     pub fn chat(
//         &self,
//         message: String,
//         system_prompt: Option<String>,
//         model: Option<String>,
//         store: Option<bool>,
//     ) -> String {
//         let api_call: APICall = APICall {
//             url: self.url.clone() + "completions",
//             content_type: String::from("application/json"),
//             authorization: format!("Bearer {}", self.token),
//             json_body: json!({
//                 "model": model.unwrap_or(String::from("gpt-4o")),
//                 "messages": [
//                     {
//                         "role": "system",
//                         "content": system_prompt.unwrap_or(String::from("You are a helpful assistant."))
//                     },
//                     {
//                         "role": "user",
//                         "content": message
//                     }
//                 ],
//                 "store": store.unwrap_or(false),
//             }),
//         };

//         let resp = self
//             .request
//             .post(api_call.url)
//             .header(CONTENT_TYPE, api_call.content_type)
//             .header(AUTHORIZATION, api_call.authorization)
//             .json(&api_call.json_body)
//             .send();

//         let body = resp
//             .expect("No response")
//             .text()
//             .expect("Failed to read response text");

//         let body: Value = serde_json::from_str(&body).expect("Failed to parse JSON");

//         if let Some(content) = body["choices"][0]["message"]["content"].as_str() {
//             return content.to_string();
//         } else {
//             return String::from("No content found");
//         }
//     }
// }
#[derive(Debug)]
pub struct APICall {
    pub url: String,
    pub content_type: String,
    pub authorization: String,
    pub json_body: serde_json::Value,
}
