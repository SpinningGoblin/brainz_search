use crate::entity_type::EntityType;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(long, short)]
    pub input: String,
    #[arg(long, short, default_value = "./output.sqlite")]
    pub output: String,
    #[arg(long, short, value_enum, default_value_t = EntityType::ReleaseGroup)]
    pub entity: EntityType,
}
