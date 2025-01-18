use crate::entities::{ArtistCredit, GenreReference, Tag};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReleaseGroup {
    pub id: String,
    pub title: String,
    #[serde(default, alias = "primary-type-id")]
    pub primary_type_id: Option<String>,
    #[serde(default, alias = "primary-type")]
    pub primary_type: Option<String>,
    #[serde(default, alias = "first-release-date")]
    pub first_release_date: Option<String>,
    #[serde(default, alias = "artist-credit")]
    pub artist_credit: Vec<ArtistCredit>,
    #[serde(default)]
    pub genres: Vec<GenreReference>,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

impl ReleaseGroup {
    pub fn artist_content(&self) -> String {
        self.artist_credit
            .iter()
            .flat_map(|artist_credit| artist_credit.name.clone())
            .collect::<Vec<_>>()
            .join("; ")
    }

    pub fn genres_content(&self) -> String {
        self.genres
            .iter()
            .map(|genre| genre.name.clone())
            .collect::<Vec<_>>()
            .join("; ")
    }
}

