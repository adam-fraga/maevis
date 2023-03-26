pub mod request {
    use std::{collections::HashMap, env};

    use super::request_entities::request_kind::RequestDataParams;
    use crate::request_builder::request_entities::*;
    use reqwest::{Body, Client};
    use serde_json;
    use tokio;

    pub async fn get_fast_answer() {
        let mut request_headers = HttpHeaders {
            api_key: env::var("OPENAI_API_KEY").unwrap(),
            url: String::from("https://api.openai.com"),
            endpoint: String::from("/v1/completions"),
            headers: HashMap::new(),
        };
        request_headers.headers.insert(
            "OPENAI_API_KEY".to_string(),
            String::from("Bearer ") + &request_headers.api_key,
        );

        let request_body = RequestDataParams {
            model: String::from("text-davinci-003"),
            prompt: String::from("Say this is a test"),
            max_tokens: 7,
            temperature: 0,
        };

        println!("API KEY: {}", &request_headers.api_key);

        let client = reqwest::Client::new();
        let res = client
            .post(request_headers.url + &request_headers.endpoint)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                request_headers.headers.get("OPENAI_API_KEY").unwrap(),
            )
            .json(&request_body)
            .send()
            .await;
    }
}

pub mod request_entities {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    // Http dormat data (api key, url & endpoint
    #[derive(Debug, Serialize)]
    pub struct HttpHeaders {
        pub api_key: String,
        pub url: String, //https://api.openai.com
        pub endpoint: String,
        pub headers: HashMap<String, String>,
    }

    // Model represents Json model from open_ai_models.json
    #[derive(Debug, Deserialize)]
    pub struct Model {
        pub id: u16,
        pub name: String,
    }

    // List that contain model (Deserialization from the json file open_ai_models.json
    #[derive(Debug, Deserialize)]
    pub struct ModelList {
        pub data: Vec<Model>,
    }

    // Data structs related to chat-gpt chat requests
    pub mod request_kind {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestDataParams {
            pub model: String,
            pub prompt: String,
            pub max_tokens: u32,
            pub temperature: u8,
            // pub top_p: u8,
            // pub n: u16,
            // pub stream: bool,
            // pub logprobs: String,
            // pub stop: char,
        }

        #[derive(Debug, Serialize)]
        pub struct ChatData {
            pub role: String,
            pub content: String,
        }

        #[derive(Debug, Serialize)]
        pub struct CompletionData {}
    }

    // Data structs related to chat-gpt completion requests
}

// User preferences for chat-gpt request
pub mod request_options {
    #[derive(Debug)]
    enum UserPreferedRequest {
        _FastAnswer,
        _Completion,
        _Translation,
        _ImageGeneration,
        _Accurate,
    }

    #[derive(Debug)]
    struct UserRequestPreferences {
        _prefered_request: String,
        _custom_model: bool,
        _custom_prompt: bool,
        _custom_token_size: bool,
        _custom_emperature: bool,
        _available_request_list: Vec<String>,
    }
}
