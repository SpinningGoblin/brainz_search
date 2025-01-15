use crate::entities::ArtistReference;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArtistCredit {
    #[serde(default)]
    pub joinphrase: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub artist: Option<ArtistReference>,
}
