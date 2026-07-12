use crate::dialogue::DialogueMessage;

use super::Character;
use super::client::CharacterPyClient;
use super::CharacterError;

pub struct CharacterSystem {
    client: CharacterPyClient,
    character: Character,
}

impl CharacterSystem {
    /// Create a brand-new character and persist it to disk.
    pub fn init(client_endpoint: String, name: String) -> Self {
        Self {
            client: CharacterPyClient::new(client_endpoint),
            character: Character::new(name),
        }
    }

    /// Try to load an existing character from disk. Returns `None` when no
    /// saved profile is found (caller should fall back to [`Self::init`]).
    pub fn load_or_init(client_endpoint: String) -> Option<Self> {
        Character::load_or_init().map(|character| Self {
            client: CharacterPyClient::new(client_endpoint),
            character,
        })
    }

    // -- Data accessors (delegate to inner Character) --

    pub fn name(&self) -> &str {
        self.character.name()
    }

    pub fn rename(&mut self, new_name: &str) {
        self.character.rename(new_name);
    }

    pub fn profile_summary(&self) -> String {
        self.character.profile_summary()
    }

    // -- Behaviour --

    /// Send the conversation history to the Python backend for profile
    /// analysis, then merge the returned insights into the character.
    pub async fn analyze_conversation(
        &mut self,
        history: &[DialogueMessage],
    ) -> Result<(), CharacterError> {
        let insights = self
            .client
            .analyze(&self.character.profile(), history)
            .await
            .map_err(|e| CharacterError::Http(e.to_string()))?;

        self.character.update_profile(insights);
        Ok(()) 
    }
}
