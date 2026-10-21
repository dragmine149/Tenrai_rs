use crate::schemas::common::{Entry, Images};
use serde::{Deserialize, Serialize};

/// Information about the character.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Character {
    /// Basic information about the character
    pub character: Entry,
    /// How many users have favorited the character
    pub favorites: u64,
    /// How important is the character role
    pub role: CharacterRole,
    /// Which VAs (voice actors) play the character.
    pub voice_actors: Vec<CharacterVA>,
}

/// How important is the character role
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CharacterRole {
    Main,
    Supporting,
}

/// Information about a character VA
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharacterVA {
    /// Which language they play the character in.
    pub language: String,
    /// The person behind the character itself.
    pub person: Person,
}

/// Information about a person.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Person {
    /// How many users have favorited this person
    pub favorites: Option<u64>,
    /// Profile Picture images
    pub images: Images,
    /// ID on MAL
    pub mal_id: u64,
    /// Name of the person
    pub name: String,
    /// Direct link to their profile on MAL
    pub url: String,
}

/// Individual entries for who helped on an anime
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnimeStaff {
    /// The person itself
    pub person: Person,
    /// What positions they were in.
    pub positions: Vec<String>,
}

/// Individual entries for who helped on a manga
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MangaAuthor {
    /// ID on MAL
    pub mal_id: u64,
    /// Name of the person
    pub name: String,
    /// The credited role
    pub role: String,
    /// Always "people" (meant to be type but well...)
    #[serde(rename = "type")]
    pub author_type: String,
    /// Direct link to MAL for this person
    pub url: String,
}

/// Information about where a manga was serialized
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MangaSerialization {
    /// ID on MAL
    pub mal_id: u64,
    /// Name of the entry
    pub name: String,
    /// Type of the serialization media
    #[serde(rename = "type")]
    pub serialize_type: String,
    /// Direct link on MAL for this item
    pub url: String,
}
