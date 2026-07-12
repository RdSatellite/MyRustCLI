use reqwest::Client;
use super::schema::{DialogueRequest, DialogueResponse};

/// HTTP client that calls the Python dialogue backend.
pub struct DialoguePyClient {
    client: Client,
    base_url: String,
}

impl DialoguePyClient {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .no_proxy()
            .build()
            .unwrap();

        Self {
            client: client,
            base_url,
        }
    }

    /// POST {base_url}/dialogue with the ChatRequest, return the ChatResponse.
    pub async fn send(&self, request: &DialogueRequest) -> Result<DialogueResponse, reqwest::Error> {
        let url = format!("{}/dialogue", self.base_url);

        self.client
            .post(&url)
            .json(request)
            .send()
            .await?
            .error_for_status()?
            .json::<DialogueResponse>()
            .await
    }
}
