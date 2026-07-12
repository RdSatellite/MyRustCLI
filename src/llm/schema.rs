use serde::{Deserialize, Serialize};

pub const SYSTEM_PROMPT: &str = r#"
You are an AI chatting with a user in a terminal.

Keep responses concise.
Reply in one or two short sentences.
Do not use Markdown.
Do not use bullet lists unless requested.
Be friendly and conversational.
"#;

#[derive(Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
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
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Deserialize)]
pub struct ChatCompletionResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: ChatMessageResponse,
}

#[derive(Deserialize)]
pub struct ChatMessageResponse {
    // pub role: String,
    pub content: String,
}
