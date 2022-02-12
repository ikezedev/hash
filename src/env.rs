extern crate dotenv;

use dotenv::dotenv;
use std::env;

const SUPPORTED_DRIVERS: [&'static str; 1] = ["pg"];

pub struct EnvVars {
    pub source: String,
    pub driver: String,
    pub metadata_url: String,
    pub query_url: String,
}

impl EnvVars {
    pub fn init() -> Self {
        dotenv().ok();
        env::var("HASURA_UTILS_SOURCE")
            .map(|source| {
                let driver = env::var("HASURA_UTILS_DDRIVER").unwrap_or("pg".to_string());
                if !SUPPORTED_DRIVERS.contains(&&driver[..]) {
                    panic!("{driver} driver is not supported yet");
                }
                (source, driver)
            })
            .map(|(source, driver)| {
                let data_url = env::var("HASURA_DATA_API_URL")
                    .expect("HASURA_DATA_API_URL env var is not set");
                Self {
                    source,
                    driver,
                    query_url: format!("{data_url}/v2/query"),
                    metadata_url: format!("{data_url}/v1/metadata"),
                }
            })
            .expect("HASURA_UTILS_SOURCE env var is not set")
    }
}
