use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OAIRequest {
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "max_tokens")]
    pub max_tokens: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OAICompletion {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "index")]
    index: u8,
    #[serde(rename = "logprobs")]
    logprobs: Option<u8>,
    #[serde(rename = "finish_reason")]
    finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OAIResponse {
    #[serde(rename = "id")]
    id: Option<String>,
    #[serde(rename = "object")]
    object: Option<String>,
    #[serde(rename = "created")]
    created: Option<u64>,
    #[serde(rename = "model")]
    model: Option<String>,
    #[serde(rename = "choices")]
    pub choices: Vec<OAICompletion>,
}
