mod cli;
mod env;
mod metadata;
mod sql;
mod types;
mod util;

use clap::StructOpt;
use cli::{App, Commands};
use env::EnvVars;
use util::HasuraUtils;

use crate::{metadata::QualifiedTable, types::RunSQLReponse};

#[tokio::main]
async fn main() {
    let cli = App::parse();
    let env = EnvVars::init();
    let client = env.makeClient();
    let app = HasuraUtils { client, env };
    let strr = r#"[{"name": "users", "schema": "public"}, {"name": "authors", "schema": "authors_schema"}]"#;

    let str_ex = format!("{{\"result_type\": \"TuplesOk\", \"result\": [[\"tables\"], [{strr}]]}}");

    match &cli.command {
        Commands::TrackTable { name, schema, all } => {
            println!("name: {name:?}, schema: {schema:?}, all: {all}");
            let state: RunSQLReponse<Vec<QualifiedTable>> =
                serde_json::from_str(&str_ex).expect("error parsing");
            let next = state.into_inner();
            println!("{next:?}");
        }
        Commands::TrackRel { name, schema, all } => {
            println!("name: {name:?}, schema: {schema:?}, all: {all}")
        }
        _ => {
            eprintln!("Sorry this functionality is not supported yet!")
        }
    }
}
