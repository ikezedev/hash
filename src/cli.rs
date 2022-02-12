use clap::{AppSettings, Parser, Subcommand};

/// A CLI for managing Hasura GraphQL Engine
#[derive(Parser)]
#[clap(name = "hasurautils")]
#[clap(about = "A CLI for managing Hasura GraphQL Engine", long_about = None)]
pub struct App {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Track table(s)
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    TrackTable {
        /// Name of the table to track
        #[clap(short, long, group = "table")]
        name: Option<String>,

        /// Schema of the table to track
        #[clap(short, long, requires = "name")]
        schema: Option<String>,

        /// If it is should track all trackable tables in the database
        #[clap(short, long, group = "table")]
        all: bool,
    },
    /// Track relationships(s)
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    TrackRel {
        /// Name of the table with the relationships to track
        #[clap(short, long, group = "relationship")]
        name: Option<String>,

        /// Schema of the table that has the relationships
        #[clap(short, long, requires = "name")]
        schema: Option<String>,

        /// If it is should track all trackable relationships in the database
        #[clap(short, long, group = "relationship")]
        all: bool,
    },
    /// Track functions(s)
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    TrackFn {
        /// Name of the function to track
        #[clap(short, long, group = "function")]
        name: Option<String>,

        /// Schema of the function to track
        #[clap(short, long, requires = "name")]
        schema: Option<String>,

        /// If it is should track all trackable functions in the database
        #[clap(short, long, group = "function")]
        all: bool,
    },
}
