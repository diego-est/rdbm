use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ResArgs {
    #[clap(subcommand)]
    /// this is a subcommand
    pub option: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Retrieve the value associated with <KEY> from resource file
    Get(Key),
    /// Set a <VALUE> for a given <KEY> in the resource file (will print to stdout the associated
    /// key-value pair)
    Set(KeyVal),
    /// Pretty print the entire resource file
    All,
}

#[derive(Debug, Args)]
pub struct Key {
    pub key: String,
}

#[derive(Debug, Args)]
pub struct KeyVal {
    pub key: String,
    pub value: String,
}
