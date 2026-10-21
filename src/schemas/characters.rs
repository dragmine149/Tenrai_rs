use crate::{
    person_connection,
    schemas::{
        common::{Entry, Images},
        people::CharacterVA,
    },
};
use serde::{Deserialize, Serialize};

/// Detailed information about the character
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharacterDetails {
    /// Information about the character
    pub about: Option<String>,
    /// Connection to an anime this character is in.
    pub anime: Option<Vec<AnimeCharacterConnection>>,
    /// How many users have favorited this character
    pub favorites: u64,
    /// Images of the character
    pub images: Images,
    /// ID of the character
    pub mal_id: u64,
    /// Connection to an manga this character is in.
    pub manga: Option<Vec<MangaCharacterConnection>>,
    /// Name of the character
    pub name: String,
    /// Kanji name of the character
    pub name_kanji: Option<String>,
    /// Nicknames that the character has
    pub nicknames: Vec<String>,
    /// URL of the character profile on MAL
    pub url: String,
    /// Who plays this character
    pub voices: Option<Vec<CharacterVA>>,
}

person_connection!(anime, character, role);
person_connection!(manga, character, role);
