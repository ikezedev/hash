mod cli;
mod env;
mod metadata;

use clap::StructOpt;
use cli::{Commands, HasuraUtils};

fn main() {
    let cli = HasuraUtils::parse();
    let _ = env::EnvVars::init();

    match &cli.command {
        Commands::TrackTable { name, schema, all } => {
            println!("name: {name:?}, schema: {schema:?}, all: {all}")
        }
        Commands::TrackRel { name, schema, all } => {
            println!("name: {name:?}, schema: {schema:?}, all: {all}")
        }
        _ => {
            eprintln!("Sorry this functionality is not supported yet!")
        }
    }
}
