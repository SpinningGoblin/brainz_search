use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(long, short)]
    pub input: String,
    #[arg(long, short, default_value = "./output.sqlite")]
    pub output: String,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(name = "rg", about = "Process release groups")]
    ReleaseGroup(ReleaseGroupArgs),
}

impl Commands {
    pub fn sql_create(&self) -> String {
        match self {
            Commands::ReleaseGroup(args) => {
                format!("CREATE VIRTUAL TABLE if not exists release_groups USING fts5(id, json UNINDEXED, artists, title, genres, tokenize=\"{}\");", args.tokenizer)
            }
        }
    }
}

#[derive(Args, Debug)]
pub struct ReleaseGroupArgs {
    #[clap(long, default_value = "Album,EP", short='t')]
    pub types: String,
    #[clap(long, default_value = "trigram case_sensitive 0 remove_diacritics 1", short='k')]
    pub tokenizer: String,
}

impl ReleaseGroupArgs {
    pub fn release_group_types(&self) -> Vec<String> {
        self.types.split(",").map(|s| s.to_string()).collect()
    }
}