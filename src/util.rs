use reqwest::{Client, Response};

use crate::error::{HasuraUtilsError, OtherError};
use crate::metadata::QualifiedTable;
use crate::sql;
use crate::types::{BulkRequest, RunSQLReponse, SQLFKRelationships, TrackTable, TrackTableArgs};
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
    pub async fn check_health(&self) -> Result<HGEHealth, HasuraUtilsError> {
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

    pub async fn get_metadata(&self) -> Result<Metadata, HasuraUtilsError> {
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

    pub async fn get_all_tables(&self) -> Result<Vec<QualifiedTable>, HasuraUtilsError> {
        let body = &self.env.get_run_sql(sql::get_all_tables_sql());
        let resp = self
            .client
            .post(&self.env.query_url)
            .json(body)
            .send()
            .await?
            .error_for_status()?
            .json::<RunSQLReponse>()
            .await?
            .into_inner::<Vec<QualifiedTable>>()?;
        Ok(resp)
    }

    pub async fn track_all_tables(&self) -> Result<(), HasuraUtilsError> {
        let metadata = self.get_metadata().await?;
        let all_tables = self.get_all_tables().await?;
        let untracked_tables = metadata.get_untracked_tables(all_tables);
        if untracked_tables.len() == 0 {
            return Err(OtherError("Database has no untracked tables").into());
        }
        let args: Vec<TrackTable> = untracked_tables
            .iter()
            .map(|table| TrackTableArgs {
                table,
                source: &self.env.source,
            })
            .map(|args| TrackTable::pg(args))
            .collect();
        let res = self
            .client
            .post(&self.env.metadata_url)
            .json(&BulkRequest::new(args))
            .send()
            .await?
            .error_for_status()?;
        println!("{res:?}");
        Ok(())
    }

    pub async fn track_table(&self, table: QualifiedTable) -> Result<Response, HasuraUtilsError> {
        let metadata = self.get_metadata().await?;
        let untracked_tables = metadata.get_untracked_tables(vec![table]);
        let args = TrackTableArgs {
            table: untracked_tables.first().expect("table is already tracked!"),
            source: &self.env.source,
        };
        let body = TrackTable::pg(args);
        let res = self
            .client
            .post(&self.env.metadata_url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;
        Ok(res)
    }

    pub async fn get_all_fk_relationships(
        &self,
    ) -> Result<Vec<SQLFKRelationships>, HasuraUtilsError> {
        let body = &self.env.get_run_sql(sql::get_all_fk_relationships());
        let resp = self
            .client
            .post(&self.env.query_url)
            .json(body)
            .send()
            .await?
            .error_for_status()?
            .json::<RunSQLReponse>()
            .await?
            .into_inner::<Vec<SQLFKRelationships>>()?;
        Ok(resp)
    }

    pub async fn track_all_relationships(&self) -> Result<Response, HasuraUtilsError> {
        let metadata = self.get_metadata().await?;
        let relationships = self.get_all_fk_relationships().await?;
        let untracked_relationships =
            metadata.get_untracked_relationships(&relationships, &self.env.source);
        if untracked_relationships.len() == 0 {
            return Err(OtherError("Database has no untracked relationships").into());
        }
        let res = self
            .client
            .post(&self.env.metadata_url)
            .json(&BulkRequest::new(untracked_relationships))
            .send()
            .await?
            .error_for_status()?;
        println!("{res:?}");
        Ok(res)
    }
}
