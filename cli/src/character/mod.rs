// character/mod.rs
mod entity;
mod client;
mod system;

pub use system::CharacterSystem;
pub use entity::{ Character, CharacterError, Profile };
