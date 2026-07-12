// character/profile.rs
use std::collections::HashMap;
use serde::{ Serialize, Deserialize };


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