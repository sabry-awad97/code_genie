use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{stdin, stdout, Write};

#[derive(Serialize, Deserialize, Debug)]
struct OAIRequest {
    #[serde(rename = "prompt")]
    prompt: String,
    #[serde(rename = "max_tokens")]
    max_tokens: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct OAIChoices {
    #[serde(rename = "text")]
    text: String,
    #[serde(rename = "index")]
    index: u8,
    #[serde(rename = "logprobs")]
    logprobs: Option<u8>,
    #[serde(rename = "finish_reason")]
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OAIResponse {
    #[serde(rename = "id")]
    id: Option<String>,
    #[serde(rename = "object")]
    object: Option<String>,
    #[serde(rename = "created")]
    created: Option<u64>,
    #[serde(rename = "model")]
    model: Option<String>,
    #[serde(rename = "choices")]
    choices: Vec<OAIChoices>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";

    let oai_token: String =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    let auth_header_val = format!("Bearer {}", oai_token);

    println!("{esc}c", esc = 27 as char);

    let mut input = String::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).expect("Failed to read line");
        let request = OAIRequest {
            prompt: format!("Generate a Sql code for the given statement. {}", input),
            max_tokens: 1000,
        };
        let response = reqwest::Client::new()
            .post(uri)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::AUTHORIZATION, &auth_header_val)
            .json(&request)
            .send()
            .await?;
        let oai_response: OAIResponse = response.json().await?;

        println!("{}", oai_response.choices[0].text);
        input.clear();
    }
}
