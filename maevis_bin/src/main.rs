use maevis::*;
use tokio::time;

#[tokio::main]
async fn main() {
    let input = "Hello chat gpt";
    let response = http_request::get_response_from_ai_assistant(input).await.unwrap();
    println!("{:?}", response);
}
