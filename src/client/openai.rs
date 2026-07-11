use super::LLMClient;
use super::messages::{ ChatMessage, ChatCompletionRequest, ChatCompletionResponse};


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
    ) -> Result<String, reqwest::Error> {

        let request = ChatCompletionRequest {
            model: "gpt-5.4".to_string(),
            messages: vec![
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                }
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

        Ok(
            response
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message.content)
            .unwrap_or_default()
        )
    }
}