// character/pyclient.rs
// 
// External Python Client for complex LLM usages.

use reqwest::Client;
use serde::{ Deserialize, Serialize};

use super::Profile;
use crate::dialogue::DialogueMessage;

/// HTTP client for Python profile extraction service.
pub struct CharacterPyClient {
    endpoint: String,
    client: Client,
}

impl CharacterPyClient {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .no_proxy()
            .build()
            .unwrap();
        
        Self {
            endpoint: format!("{}/profile/extract", base_url.trim_end_matches('/')),
            client: client,
        }
    }

    pub async fn analyze(
        &self,
        profile: &Profile,
        history: &[DialogueMessage],
    ) -> Result<Profile, reqwest::Error> {
        let req = ExtractProfileRequest {
            profile: profile.clone(),
            history: history.to_vec(),
        };

        let rsp = self
            .client
            .post(&self.endpoint)
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json::<ExtractProfileResponse>()
            .await?;

        Ok(rsp.new_insight)
    }
}

// --- Schemas --- //

#[derive(Serialize)]
struct ExtractProfileRequest {
    profile: Profile,
    history: Vec<DialogueMessage>,
}

#[derive(Deserialize)]
struct ExtractProfileResponse {
    new_insight: Profile
}