use crate::schemas::{
    common::{Entry, Images},
    people::CharacterVA,
};
use serde::{Deserialize, Serialize};

/// Detailed information about the character
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharacterDetails {
    /// Information about the character
    pub about: Option<String>,
    /// Connection to an anime this character is in.
    #[serde(default)]
    pub anime: Vec<AnimeCharacterMediaConnection>,
    /// How many users have favorited this character
    pub favorites: u64,
    /// Images of the character
    pub images: Images,
    /// ID of the character
    pub mal_id: u64,
    /// Connection to an manga this character is in.
    #[serde(default)]
    pub manga: Vec<MangaCharacterMediaConnection>,
    /// Name of the character
    pub name: String,
    /// Kanji name of the character
    pub name_kanji: Option<String>,
    /// Nicknames that the character has
    pub nicknames: Vec<String>,
    /// URL of the character profile on MAL
    pub url: String,
    /// Who plays this character
    #[serde(default)]
    pub voices: Vec<CharacterVA>,
}

/// Connects a character to an anime
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnimeCharacterMediaConnection {
    /// The anime they are in.
    pub anime: Entry,
    ///What role they play.
    pub role: String,
}

/// Connects a character to an manga
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MangaCharacterMediaConnection {
    /// The manga they are in.
    pub manga: Entry,
    ///What role they play.
    pub role: String,
}
