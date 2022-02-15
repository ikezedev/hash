mod cli;
mod env;
mod error;
mod metadata;
mod sql;
mod types;
mod util;

use clap::StructOpt;
use cli::{App, Commands};
use env::EnvVars;
use util::HasuraUtils;

use crate::metadata::QualifiedTable;

#[tokio::main]
async fn main() {
    let cli = App::parse();
    let env = EnvVars::init();
    let client = env.make_client();
    let app = HasuraUtils { client, env };

    match &cli.command {
        Commands::TrackTable {
            name,
            schema,
            all,
            ignore,
            list,
        } => {
            if *all {
                let res = app.track_all_tables(ignore).await;
                println!("{res:?}");
            } else if *list {
                let res = app.get_all_tables().await;
                println!("{res:?}");
            } else {
                let res = app
                    .track_table(QualifiedTable {
                        name: name.as_ref().unwrap().to_string(),
                        schema: schema.as_ref().unwrap().to_string(),
                    })
                    .await;
                println!("{res:?}");
            }
        }
        Commands::TrackRel {
            name: _,
            schema: _,
            all,
        } => {
            if *all {
                let res = app.track_all_relationships().await;
                println!("\n{res:?}");
            } else {
                println!("Not implemented yet!");
            }
        }
        _ => {
            println!("Sorry this functionality is not supported yet!")
        }
    }
}
