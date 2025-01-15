use clap::ValueEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, ValueEnum)]
pub enum EntityType {
    /// A release group from MusicBrainz. For more information see: https://musicbrainz.org/doc/Release_Group
    ReleaseGroup,
}

impl EntityType {
    pub fn sql_create(&self) -> &'static str {
        match self {
            EntityType::ReleaseGroup => {
                r#"
        CREATE VIRTUAL TABLE if not exists release_groups USING fts5(
            id UNINDEXED,
            json UNINDEXED,
            artists,
            title,
            genres,
        );
        "#
            }
        }
    }
}
