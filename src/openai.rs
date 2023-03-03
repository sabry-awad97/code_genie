use dotenv::dotenv;
use lru::LruCache;
use std::{env, num::NonZeroUsize};

use crate::model::{OAIRequest, OAIResponse};

const CACHE_SIZE: usize = 100;

pub struct CodeCache {
    cache: LruCache<String, String>,
}

impl CodeCache {
    pub fn new() -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(CACHE_SIZE).unwrap()),
        }
    }

    pub fn put(&mut self, prompt: String, code: String) {
        self.cache.put(prompt, code);
    }

    pub fn get(&mut self, prompt: &str) -> Option<String> {
        self.cache.get(prompt).cloned()
    }
}

pub struct OpenAI {
    pub api_key: String,
    pub cache: CodeCache,
}

impl OpenAI {
    pub fn new() -> Result<Self, String> {
        dotenv().ok();
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| String::from("Please set OPENAI_API_KEY environment variable"))?;

        let cache = CodeCache::new();

        Ok(Self { api_key, cache })
    }

    pub async fn generate_code(&mut self, prompt: &str) -> Result<String, String> {
        if let Some(cached_result) = self.cache.get(prompt) {
            return Ok(cached_result);
        }

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

        let result = response.choices[0].text.clone();

        self.cache.put(prompt.to_owned(), result.clone());

        Ok(result)
    }
}
