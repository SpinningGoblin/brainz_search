use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GenreReference {
    pub id: String,
    pub name: String,
}
