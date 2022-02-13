use reqwest::{Client, Response};

use crate::metadata::QualifiedTable;
use crate::sql;
use crate::types::{BulkRequest, RunSQLReponse, TrackTable, TrackTableArgs};
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

    pub async fn get_all_tables(&self) -> Result<Vec<QualifiedTable>, Box<dyn std::error::Error>> {
        let body = &self.env.getRunSQL(sql::getAllTablesSql());
        let resp = self
            .client
            .post(&self.env.query_url)
            .json(body)
            .send()
            .await?
            .json::<RunSQLReponse<Vec<QualifiedTable>>>()
            .await?;
        Ok(resp.into_inner())
    }

    pub async fn trackAllTables(&self) -> Result<Response, Box<dyn std::error::Error>> {
        let metadata = self.get_metadata().await?;
        let all_tables = self.get_all_tables().await?;
        let untracked_tables = metadata.get_untracked_tables(all_tables);
        let args: Vec<TrackTable> = untracked_tables
            .into_iter()
            .map(|table| TrackTableArgs {
                table,
                source: &self.env.source,
            })
            .map(|args| TrackTable::new(args))
            .collect();
        let res = self
            .client
            .post(&self.env.metadata_url)
            .json(&BulkRequest::new(args))
            .send()
            .await?
            .error_for_status()?;
        println!("{res:?}");
        Ok(res)
    }
}
