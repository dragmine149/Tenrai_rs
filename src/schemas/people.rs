use crate::{
    person_connection,
    schemas::common::{Entry, Images},
};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

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
    #[serde(default)]
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
    pub person: BasicPerson,
}

/// Information about a person.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BasicPerson {
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
    pub person: BasicPerson,
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
    pub role: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Display, Clone)]
#[serde(deny_unknown_fields)]
#[strum(serialize_all = "snake_case")]
/// How to order the people list
pub enum PeopleOrder {
    MalId,
    Name,
    Birthday,
    Favorites,
}

/// Detailed information about a person
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Person {
    /// Core schema represnenting a person
    pub about: Option<String>,
    /// Alternate names, Nicknames or Kanji Aliases
    pub alternate_names: Vec<String>,
    /// Anime this person has a position in
    ///
    /// Requires full details
    pub anime: Option<Vec<AnimePersonConnection>>,
    /// Birthday of the person.
    ///
    /// Birthday in ISO 8601 format, as on Jikan. A birthday MAL gives only as a month and year is shown on the 1st of that month. One given only as a year, or with no year, is null.
    pub birthday: Option<String>,
    /// Family name of the person (surname)
    pub family_name: Option<String>,
    /// How many users have favorited this anime
    pub favorites: Option<u64>,
    /// Given name of the person (name)
    pub given_name: Option<String>,
    /// Portrait Images of the person
    pub images: Images,
    /// MAL ID of the person
    pub mal_id: u64,
    /// Anime this person has a position in
    ///
    /// Requires full details
    pub manga: Option<Vec<MangaPersonConnection>>,
    /// Full name of the person, japanese style (Family, Given)
    pub name: String,
    /// Direct profile page URL on MAL
    pub url: String,
    /// Connection to characters and anime this person has voiced/in
    ///
    /// Requires full details
    pub voices: Vec<VoiceConnection>,
    /// Official website of the person.
    pub website_url: Option<String>,
}

person_connection!(anime, person, position);
person_connection!(manga, person, position);

#[derive(Debug, Serialize, Deserialize)]
pub struct VoiceConnection {
    pub anime: Entry,
    pub character: Entry,
    pub role: String,
}
