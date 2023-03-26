use reqwest::header::USER_AGENT;

pub async fn get_fast_answer() {
    let client = reqwest::Client::new();
    let _res = client
        .get("https://api.openai.com/v1/models")
        .header(
            USER_AGENT,
            "Authorization: Bearer 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",
        )
        .send();
}
pub async fn get_specific_answer() {}
pub async fn get_accurate_answer() {}
pub async fn get_translation() {}
pub async fn get_image_generation() {}
pub async fn get_autocompletion() {}
