use colored::*;
use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct OAIRequest {
    #[serde(rename = "prompt")]
    prompt: String,
    #[serde(rename = "max_tokens")]
    max_tokens: u16,
}

#[derive(Debug, Deserialize, Serialize)]
struct OAICompletion {
    #[serde(rename = "text")]
    text: String,
    #[serde(rename = "index")]
    index: u8,
    #[serde(rename = "logprobs")]
    logprobs: Option<u8>,
    #[serde(rename = "finish_reason")]
    finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
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
    choices: Vec<OAICompletion>,
}

#[derive(Debug)]
enum OAIError {
    Api(reqwest::Error),
    Json(serde_json::Error),
    Unknown(String),
}

impl From<reqwest::Error> for OAIError {
    fn from(error: reqwest::Error) -> Self {
        OAIError::Api(error)
    }
}

impl From<serde_json::Error> for OAIError {
    fn from(error: serde_json::Error) -> Self {
        OAIError::Json(error)
    }
}

fn get_input() -> Result<String, io::Error> {
    let mut input = String::new();
    print!("{}>{}", "Sql:".green(), " ".blue());
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

async fn generate_sql_code(prompt: &str, api_key: &str) -> Result<String, OAIError> {
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";
    let request = OAIRequest {
        prompt: prompt.to_owned(),
        max_tokens: 1000,
    };
    let auth_header_val = format!("Bearer {}", api_key);

    let response = reqwest::Client::new()
        .post(uri)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::AUTHORIZATION, &auth_header_val)
        .json(&request)
        .send()
        .await?
        .json::<OAIResponse>()
        .await?;

    Ok(response.choices[0].text.clone())
}

#[tokio::main]
async fn main() -> Result<(), OAIError> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        OAIError::Unknown("Please set OPENAI_API_KEY environment variable".to_owned())
    })?;
    loop {
        match get_input() {
            Ok(input) if input.trim().is_empty() => continue,
            Ok(input) => match generate_sql_code(
                &format!("Generate a SQL code for the given statement. {}", input),
                &api_key,
            )
            .await
            {
                Ok(sql_code) => println!("{}", sql_code),
                Err(OAIError::Api(error)) => println!("{} {}", "API Error:".red().bold(), error),
                Err(OAIError::Json(error)) => println!("{} {}", "JSON Error:".red().bold(), error),
                Err(OAIError::Unknown(error)) => {
                    println!("{} {}", "Unknown Error:".red().bold(), error)
                }
            },
            Err(error) => return Err(OAIError::Unknown(format!("Input Error: {}", error))),
        }
    }
}
