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
