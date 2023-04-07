use reqwest::Client;
use serde_json::json;
use std::env;

pub async fn get_response_from_ai_assistant(
    message: &str,
    temperature: Option<f64>,
    _token: Option<u32>,
    _model: Option<&str>,
) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let _temperature = 0.7;
    let model = "gpt-3.5-turbo";
    let api_key = format!("Bearer {}", env::var("OPENAI_API_KEY").unwrap());
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", &api_key)
        .json(&json!({
            "model": model,
            "messages": [{"role": "user", "content": message}],
            "temperature": temperature,
        }))
        .send()
        .await?;
    let result = response.text().await?;
    println!("{}", &result);
    Ok(result)
}

pub async fn get_specific_answer() {}
pub async fn get_accurate_answer() {}
pub async fn get_translation() {}
pub async fn get_image_generation() {}
pub async fn get_autocompletion() {}
