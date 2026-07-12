mod client;
mod schema;

use client::DialoguePyClient;
use schema::DialogueRequest;

pub use schema::DialogueMessage;

// --- Error --- //

#[derive(Debug)]
pub enum DialogueError {
    Http(String),
}

impl std::fmt::Display for DialogueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DialogueError::Http(msg) => write!(f, "Dialogue HTTP error: {msg}"),
        }
    }
}

impl std::error::Error for DialogueError {}

// --- System --- //

/// Owns the conversation history and the Python dialogue backend client.
pub struct DialogueSystem {
    client: DialoguePyClient,
    history: Vec<DialogueMessage>,
}

impl DialogueSystem {
    pub fn new(backend_url: String) -> Self {
        Self {
            client: DialoguePyClient::new(backend_url),
            history: Vec::new(),
        }
    }

    /// Send the user's message to the Python backend and return the
    /// assistant's reply. Both the user message and the reply are appended to
    /// the internal conversation history.
    pub async fn send(&mut self, message: &str) -> Result<String, DialogueError> {
        self.history.push(DialogueMessage::user(message));

        let request = DialogueRequest {
            model: String::from(""), // Default
            messages: self.history.clone(),
        };

        let response = self
            .client
            .send(&request)
            .await
            .map_err(|e| DialogueError::Http(e.to_string()))?;

        let reply = response.message.content;
        self.history.push(DialogueMessage::assistant(&reply));

        Ok(reply)
    }

    /// Expose the full conversation history (for CharacterSystem analysis).
    pub fn history(&self) -> &[DialogueMessage] {
        &self.history
    }
}
