mod schema;
mod openai;

use std::fmt;
pub use schema::{ ChatMessage };
pub use openai::OpenAIClient;

pub trait LLMClient {
    async fn invoke(
        &self, 
        prompt: &str
    ) -> Result<String, LLMError>;
}

#[derive(Debug)]
pub enum LLMError {
    Network( reqwest::Error ),
    EmptyResponse,
}

impl fmt::Display for LLMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LLMError::Network(e) => write!(f, "Network error: {e}"),
            LLMError::EmptyResponse => write!(f, "Empty response from LLM"),
        }
    }
}

impl std::error::Error for LLMError {}

impl From<reqwest::Error> for LLMError {
    fn from(err: reqwest::Error) -> Self {
        LLMError::Network(err)
    }
}
