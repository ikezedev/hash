extern crate dotenv;
use reqwest::{header, Client};

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{env, str::FromStr};

const SUPPORTED_DRIVERS: [&'static str; 1] = ["pg"];

pub struct EnvVars {
    pub source: String,
    pub driver: String,
    pub metadata_url: String,
    pub query_url: String,
    pub admin_secret: String,
    pub healthz: String,
}

#[derive(Debug, Serialize)]
pub struct RunSQL<'a> {
    r#type: &'a str,
    args: RunSQLArgs<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RunSQLArgs<'a> {
    source: &'a str,
    sql: &'a str,
    cascade: bool,
    read_only: bool,
}

impl<'a> Default for RunSQL<'a> {
    fn default() -> Self {
        Self {
            r#type: "run_sql",
            args: Default::default(),
        }
    }
}

impl<'a> Default for RunSQLArgs<'a> {
    fn default() -> Self {
        Self {
            source: "default",
            sql: Default::default(),
            cascade: false,
            read_only: true,
        }
    }
}

impl EnvVars {
    pub fn init() -> Self {
        dotenv().ok();
        let source =
            env::var("HASURA_UTILS_SOURCE").expect("HASURA_UTILS_SOURCE env var is not set");
        let driver = env::var("HASURA_UTILS_DDRIVER").unwrap_or("pg".to_string());
        let data_url =
            env::var("HASURA_DATA_API_URL").expect("HASURA_DATA_API_URL env var is not set");
        let admin_secret =
            env::var("HASURA_ADMIN_SECRET").expect("HASURA_ADMIN_SECRET env var is not set");
        if !SUPPORTED_DRIVERS.contains(&&driver[..]) {
            panic!("{driver} driver is not supported yet");
        }
        Self {
            source,
            driver,
            admin_secret,
            query_url: format!("{data_url}/v2/query"),
            metadata_url: format!("{data_url}/v1/metadata"),
            healthz: format!("{data_url}/healthz"),
        }
    }

    pub fn makeClient(&self) -> Client {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-HASURA-ADMIN-SECRET",
            header::HeaderValue::from_str(&self.admin_secret).unwrap(),
        );
        let h = &self.healthz;
        println!("{h}");
        let a = &self.admin_secret;
        println!("{a}");

        return Client::builder()
            .default_headers(headers)
            .build()
            .expect("unable to construct client");
    }

    pub fn getRunSQL<'a>(&'a self, sql: &'a str) -> RunSQL<'a> {
        RunSQL {
            args: RunSQLArgs {
                source: &self.source,
                sql: sql.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
