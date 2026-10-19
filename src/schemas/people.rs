use crate::schemas::common::{Entry, Images};
use serde::{Deserialize, Serialize};

/// Information about the character.
#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub character: Entry,
    pub favorites: u64,
    pub role: CharacterRole,
    pub voice_actors: Vec<CharacterVA>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CharacterRole {
    Main,
    Supporting,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterVA {
    pub language: String,
    pub person: Person,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub favorites: Option<u64>,
    pub images: Images,
    pub mal_id: u64,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeStaff {
    pub person: Person,
    pub positions: Vec<String>,
}
