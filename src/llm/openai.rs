use super::{ LLMClient, LLMError};
use super::schema::{ ChatMessage, ChatCompletionRequest, ChatCompletionResponse, SYSTEM_PROMPT};

pub struct OpenAIClient {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl OpenAIClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url, 
            api_key,
        }
    }
}

impl LLMClient for OpenAIClient {
    async fn invoke(
        &self, 
        prompt: &str
    ) -> Result<String, LLMError> {

        let request = ChatCompletionRequest {
            model: "deepseek-v4-pro".to_string(),
            messages: vec![
                ChatMessage::system(SYSTEM_PROMPT),
                ChatMessage::user(prompt)
            ],
        };

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<ChatCompletionResponse>()
            .await?;

        response
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message.content)
            .ok_or(LLMError::EmptyResponse)
    }
}