mod cli;
mod env;
mod metadata;
mod util;

use clap::StructOpt;
use cli::{App, Commands};
use env::EnvVars;
use util::HasuraUtils;

#[tokio::main]
async fn main() {
    let cli = App::parse();
    let env = EnvVars::init();
    let client = env.makeClient();
    let app = HasuraUtils { client, env };

    match &cli.command {
        Commands::TrackTable { name, schema, all } => {
            println!("name: {name:?}, schema: {schema:?}, all: {all}");
            let state = app.get_metadata().await;
            println!("{state:?}");
        }
        Commands::TrackRel { name, schema, all } => {
            println!("name: {name:?}, schema: {schema:?}, all: {all}")
        }
        _ => {
            eprintln!("Sorry this functionality is not supported yet!")
        }
    }
}
