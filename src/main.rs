use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};

#[derive(Serialize, Debug)]
struct OAIRequest {
    prompt: String,
    max_tokens: u16,
}

#[derive(Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

#[derive(Deserialize, Debug)]
struct OAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";

    let oai_token: String = env::var("OPENAI_API_KEY").unwrap();
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
        let body = Body::from(serde_json::to_vec(&request)?);

        let sp = Spinner::new(&Spinners::Dots12, "\t\tOpenAI is Thinking...".into());

        let request = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &auth_header_val)
            .body(body)
            .unwrap();
        let response = client.request(request).await?;
        let body = hyper::body::aggregate(response).await?;
        let oai_response: OAIResponse = serde_json::from_reader(body.reader())?;

        sp.stop();
        println!("{}", oai_response.choices[0].text);
        input.clear();
    }
}
