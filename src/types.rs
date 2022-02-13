use serde::{Deserialize, Serialize, __private::de};

use crate::metadata::QualifiedTable;

#[derive(Debug, Deserialize)]
#[serde(tag = "result_type", content = "result")]
pub enum RunSQLReponse<T> {
    TuplesOk(((String,), (T,))),
    CommandOK,
}

impl<T> RunSQLReponse<T>
where
    T: Default,
{
    pub fn into_inner(self) -> T {
        match self {
            RunSQLReponse::TuplesOk(data) => (data.1).0,
            RunSQLReponse::CommandOK => T::default(),
        }
    }
}

pub trait MetadataRequest {}

#[derive(Debug, Serialize)]
pub struct TrackTable<'a> {
    r#type: &'a str,
    args: TrackTableArgs<'a>,
}
impl<'a> TrackTable<'a> {
    pub fn new(args: TrackTableArgs<'a>) -> Self {
        Self {
            r#type: "pg_track_table",
            args,
        }
    }
}

impl<'a> MetadataRequest for TrackTable<'a> {}

#[derive(Debug, Serialize)]
pub struct TrackTableArgs<'a> {
    pub(crate) source: &'a str,
    pub(crate) table: QualifiedTable,
}

#[derive(Debug, Serialize)]
pub struct BulkRequest<'a, T: MetadataRequest> {
    r#type: &'a str,
    args: Vec<T>,
}

impl<'a, T: MetadataRequest> BulkRequest<'a, T> {
    pub fn new(args: Vec<T>) -> Self {
        BulkRequest {
            r#type: "bulk",
            args,
        }
    }
}
