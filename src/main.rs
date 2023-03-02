use colored::*;
use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
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

struct OpenAI {
    api_key: String,
}

impl OpenAI {
    fn new() -> Result<Self, String> {
        dotenv().ok();
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| String::from("Please set OPENAI_API_KEY environment variable"))?;

        Ok(Self { api_key })
    }

    async fn generate_code(&self, prompt: &str) -> Result<String, String> {
        let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";
        let request = OAIRequest {
            prompt: prompt.to_owned(),
            max_tokens: 1000,
        };
        let auth_header_val = format!("Bearer {}", self.api_key);

        let response = reqwest::Client::new()
            .post(uri)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::AUTHORIZATION, &auth_header_val)
            .json(&request)
            .send()
            .await
            .map_err(|err| format!("API Error: {}", err))?
            .json::<OAIResponse>()
            .await
            .map_err(|err| format!("JSON Error: {}", err))?;

        Ok(response.choices[0].text.clone())
    }
}

struct SqlGenerator {
    openai: OpenAI,
}

impl SqlGenerator {
    fn new() -> Result<Self, String> {
        let openai = OpenAI::new()?;
        Ok(Self { openai })
    }

    fn get_input() -> Result<String, io::Error> {
        let mut input = String::new();
        print!("{}>{}", "Sql:".green(), " ".blue());
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    async fn generate_and_print_sql_code(&self, input: &str) {
        let mut sp = Spinner::new(Spinners::Dots12, "\t\tOpenAI is Thinking...".into());
        match self
            .openai
            .generate_code(&format!(
                "Generate a SQL code for the given statement. {}",
                input
            ))
            .await
        {
            Ok(sql_code) => {
                // stopping the spinner
                sp.stop();
                println!("{}", sql_code)
            }
            Err(err) => {
                sp.stop();
                println!("{} {}", "Error:".red().bold(), err)
            }
        }
    }

    async fn run(&self) -> Result<(), String> {
        println!("{esc}c", esc = 27 as char);

        loop {
            match Self::get_input() {
                Ok(input) if input.trim().is_empty() => continue,
                Ok(input) => self.generate_and_print_sql_code(&input).await,
                Err(error) => return Err(format!("Error: {}", error)),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let sql_generator = SqlGenerator::new()?;
    sql_generator.run().await?;
    Ok(())
}
