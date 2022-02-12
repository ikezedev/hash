use reqwest::Client;

use crate::{env::EnvVars, metadata::Metadata};

pub struct HasuraUtils {
    pub client: Client,
    pub env: EnvVars,
}

#[derive(Debug)]
pub enum HGEHealth {
    Ok,
    Inconsistent,
    Error,
}

impl HasuraUtils {
    pub async fn check_health(&self) -> Result<HGEHealth, Box<dyn std::error::Error>> {
        let res = &self
            .client
            .get(&self.env.healthz)
            .send()
            .await?
            .text()
            .await?;
        let state = match &res[..] {
            "OK" => HGEHealth::Ok,
            "ERROR" => HGEHealth::Error,
            _ => HGEHealth::Inconsistent,
        };
        Ok(state)
    }

    pub async fn get_metadata(&self) -> Result<Metadata, Box<dyn std::error::Error>> {
        let res = self
            .client
            .post(&self.env.metadata_url)
            .body(r#"{"type": "export_metadata", "args": {}}"#)
            .send()
            .await?
            .json::<Metadata>()
            .await?;
        Ok(res)
    }
}
