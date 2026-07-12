use std::fs;
use serde::{Deserialize, Serialize};
use super::Profile;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    name: String,
    profile: Profile,
}

impl Character {
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
        match fs::read_to_string("data/user_profile.json") {
            Ok(json_content) => serde_json::from_str(&json_content).ok(),
            Err(_) => None,
        }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn profile(&self) -> &Profile { &self.profile }

    pub fn rename(&mut self, new_name: &str) {
        let new_name = new_name.trim().to_string();
        fs::write("data/names.txt", &new_name).expect("Failed to write file");
        self.name = new_name;
    }

    pub fn profile_summary(&self) -> String {
        self.profile.summary()
    }

    pub fn update_profile(&mut self, insights: Profile) {
        self.profile.merge(insights);
        self.save_to_disk();
    }

    pub fn save_to_disk(&self) {
        let json_string =
            serde_json::to_string_pretty(self).expect("Failed to serialize character data");
        fs::write("data/user_profile.json", json_string)
            .expect("Failed to write profile storage file");
    }
}
