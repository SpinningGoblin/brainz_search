use crate::entities::{GenreReference, Tag};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArtistReference {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub genres: Vec<GenreReference>,
    #[serde(default, alias = "type")]
    pub artist_type: Option<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
}
