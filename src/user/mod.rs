// user/mod.rs
use std::{ fs };

pub struct User {
    name: String,
}

impl User {
    pub fn load_or_init() -> Option<Self>{
        match fs::read_to_string("names.txt") {
            Ok(name) => {
                let name = name.trim().to_string();
                println!("Welcome back, {name}!");
                Some(Self { name })
            }
            Err(_) => None,
        }
    }

    pub fn new(name: String) -> Self {
        let name = name.trim().to_string();
        fs::write("name.txt", &name).expect("Failed to write file");
        println!("Hello, {name}!");
        Self { name }
    }

    pub fn name(&self) -> &str {&self.name}

    pub fn rename(&mut self, new_name: &str) {
        let new_name = new_name.trim().to_string();

        fs::write("names.txt", &new_name)
            .expect("Failed to write file");

        self.name = new_name;
    }
}