pub trait LLMClient {
    async fn invoke(&self, prompt: &str) 
        -> Result<String, reqwest::Error>;
}