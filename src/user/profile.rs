use std::collections::HashMap;
use serde::{ Serialize, Deserialize };

use crate::llm::{self, LLMClient};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    data: HashMap<String, String>
}

impl Profile {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn from_map(data: HashMap<String, String>) -> Self {
        Self { data }
    }

    pub fn merge(&mut self, other: Profile) {
        for (topic, detail) in other.data {
            self.data.insert(topic, detail);
        }
    }

    pub fn summary(&self) -> String {
        self.data
            .iter()
            .map(|(k, v)| format!("- {}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Extract user profile information from conversation history
/// Lossy compression using LLM technology
pub async fn extract_profile(
    client: &impl LLMClient,
    history: &[llm::ChatMessage],
) -> Result<Profile, llm::LLMError> {
    let mut transcript = String::new();

    for msg in history {
        transcript.push_str(
            &format!("{}: {}\n", msg.role, msg.content)
        );
    }

    let instruction = format!(
        "Analyze this conversation. Extract permanent user information, \
        such as traits, favorite programming languages, learning preferences, \
        and technical interests.\n\n\
        Return ONLY a valid JSON object matching this schema:\n\
        {{\"Topic\": \"Observation\"}}\n\n\
        If nothing notable exists, return {{}}.\n\n\
        Conversation:\n{transcript}"
    );

    let response = client.invoke(&instruction).await?;

    let clean_json = response
        .trim()
        .trim_matches('`')
        .trim_start_matches("json")
        .trim();

    let map: HashMap<String, String> =
        serde_json::from_str(clean_json)
            .unwrap_or_default();

    Ok(Profile::from_map(map))
}