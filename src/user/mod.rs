// user/mod.rs
mod profile;

use std::collections::HashMap;
use std::{ fs };

use crate::llm;
use profile::Profile;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    name: String,
    profile: Profile
}

impl User {
    pub fn new(name: String) -> Self {
        let instance = Self {
            name: name.trim().to_string(),
            profile: Profile::new(),
        };
        instance.save_to_disk();
        println!("Hello, {}!", instance.name);
        instance
    }

    pub fn load_or_init() -> Option<Self> {
        match fs::read_to_string("user_profile.json") {
            Ok(json_content) => {
                serde_json::from_str(&json_content).ok()
            }
            Err(_) => None,
        }
    }

    pub fn name(&self) -> &str {&self.name}
    pub fn rename(&mut self, new_name: &str) {
        let new_name = new_name.trim().to_string();

        fs::write("names.txt", &new_name)
            .expect("Failed to write file");

        self.name = new_name;
    }

    pub fn update_profile(
        &mut self,
        insights: Profile,
    ) {
        self.profile.merge(insights);
        self.save_to_disk();
    }

    fn save_to_disk(&self) {
        let json_string = serde_json::to_string_pretty(self)
            .expect("Failed to serialize user data structures");
        fs::write("user_profile.json", json_string)
            .expect("Failed to write profile storage file");
    }
}

pub async fn update_user_profile(
    user: &mut User,
    history: &[llm::ChatMessage],
    client: &impl llm::LLMClient,
) -> Result<(), llm::LLMError> {
    
    let insights = profile::extract_profile(
        client,
        history
    ).await?;

    user.update_profile(insights);

    Ok(())
}