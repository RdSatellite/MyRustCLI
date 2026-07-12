use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DialogueMessage {
    pub role: String,
    pub content: String,
}

impl DialogueMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".into(),
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".into(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".into(),
            content: content.into(),
        }
    }
}

#[derive(Serialize)]
pub struct DialogueRequest {
    pub model: String,
    pub messages: Vec<DialogueMessage>,
}

#[derive(Deserialize)]
pub struct DialogueResponse {
    pub message: DialogueMessage
}
