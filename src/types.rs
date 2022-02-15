use std::{collections::HashMap, fmt::Display};

use console::style;
use inflector::Inflector;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::metadata::{
    ArrayRelUsing, ArrayRelUsingFKeyOn, ArrayRelationships, ObjRelUsing, ObjectRelationships,
    QualifiedTable,
};

#[derive(Debug, Deserialize)]
#[serde(tag = "result_type", content = "result")]
pub enum RunSQLReponse {
    TuplesOk(((String,), (String,))),
    CommandOK,
}

impl RunSQLReponse {
    pub fn into_inner<T>(&self) -> Result<T, serde_json::Error>
    where
        T: Default + DeserializeOwned,
    {
        match self {
            RunSQLReponse::TuplesOk(data) => serde_json::from_str::<'_, T>(&data.1 .0),
            RunSQLReponse::CommandOK => Ok(T::default()),
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
    pub fn pg(args: TrackTableArgs<'a>) -> Self {
        Self {
            r#type: "pg_track_table",
            args,
        }
    }
}

pub enum RelType {
    Array,
    Object,
}

#[derive(Debug, Serialize)]
pub struct CreateObjectRelationship<'a> {
    r#type: &'static str,
    args: CreateObjectRelationshipArgs<'a>,
}

impl<'a> CreateObjectRelationship<'a> {
    fn pg(args: CreateObjectRelationshipArgs<'a>) -> Self {
        Self {
            r#type: "pg_create_object_relationship",
            args,
        }
    }
}

impl<'a> CreateArrayRelationship<'a> {
    fn pg(args: CreateArrayRelationshipArgs<'a>) -> Self {
        Self {
            r#type: "pg_create_array_relationship",
            args,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateArrayRelationship<'a> {
    r#type: &'static str,
    args: CreateArrayRelationshipArgs<'a>,
}

#[derive(Debug, Serialize)]
pub struct CreateObjectRelationshipArgs<'a> {
    #[serde(flatten)]
    rel: ObjectRelationships,
    source: &'a str,
    table: QualifiedTable,
}

#[derive(Debug, Deserialize)]
pub struct SQLFKRelationship {
    pub table_name: String,
    pub table_schema: String,
    pub constraint_name: String,
    pub ref_table_schema: String,
    pub ref_table_name: String,
    pub column_mapping: HashMap<String, String>,
}

impl Display for SQLFKRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = self.column_mapping.keys().nth(0).unwrap();
        let value = self.column_mapping.get(key).unwrap();
        let obj = format!(
            "{} {} {}  -  {}.{} --> {}.{}",
            style(&self.table_name).bold(),
            style("-->").bold(),
            style(&self.ref_table_name).bold(),
            self.table_name,
            style(key).magenta(),
            self.ref_table_name,
            style(value).magenta()
        );
        let arr = format!(
            "{} {} {} {} {}  -  {}.{} --> {}.{}",
            style(&self.ref_table_name).bold(),
            style("-->").bold(),
            style("[").bold(),
            style(&self.table_name).bold(),
            style("]").bold(),
            self.table_name,
            style(key).magenta(),
            self.ref_table_name,
            style(value).magenta(),
        );
        write!(f, "{}\n\n{}", &obj, &arr)
    }
}

impl SQLFKRelationship {
    pub fn get_relationships<'a>(
        &'a self,
        source: &'a str,
    ) -> (CreateObjectRelationship<'a>, CreateArrayRelationship<'a>) {
        let key = self.column_mapping.keys().nth(0).unwrap();
        let obj_args = CreateObjectRelationshipArgs {
            rel: ObjectRelationships {
                name: Inflector::to_singular(&self.ref_table_name),
                using: ObjRelUsing {
                    foreign_key_constraint_on: key.to_string(),
                },
            },
            source,
            table: QualifiedTable {
                name: self.table_name.to_string(),
                schema: self.table_schema.to_string(),
            },
        };
        let arr_args = CreateArrayRelationshipArgs {
            source,
            table: QualifiedTable {
                name: self.ref_table_name.to_string(),
                schema: self.ref_table_schema.to_string(),
            },
            rel: ArrayRelationships {
                name: Inflector::to_plural(&self.table_name),
                using: ArrayRelUsing {
                    foreign_key_constraint_on: ArrayRelUsingFKeyOn {
                        column: key.to_string(),
                        table: QualifiedTable {
                            name: self.table_name.to_string(),
                            schema: self.table_schema.to_string(),
                        },
                    },
                },
            },
        };
        (
            CreateObjectRelationship::pg(obj_args),
            CreateArrayRelationship::pg(arr_args),
        )
    }
}

#[derive(Debug, Serialize)]
pub struct CreateArrayRelationshipArgs<'a> {
    #[serde(flatten)]
    rel: ArrayRelationships,
    source: &'a str,
    table: QualifiedTable,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CreateRelationship<'a> {
    Object(CreateObjectRelationship<'a>),
    Array(CreateArrayRelationship<'a>),
}

impl<'a> MetadataRequest for CreateRelationship<'a> {}

impl<'a> From<CreateObjectRelationship<'a>> for CreateRelationship<'a> {
    fn from(rel: CreateObjectRelationship<'a>) -> Self {
        CreateRelationship::Object(rel)
    }
}

impl<'a> From<CreateArrayRelationship<'a>> for CreateRelationship<'a> {
    fn from(rel: CreateArrayRelationship<'a>) -> Self {
        CreateRelationship::Array(rel)
    }
}

impl<'a> MetadataRequest for TrackTable<'a> {}

#[derive(Debug, Serialize)]
pub struct TrackTableArgs<'a> {
    pub(crate) source: &'a str,
    pub(crate) table: &'a QualifiedTable,
}

#[derive(Debug, Serialize)]
pub struct BulkRequest<T: MetadataRequest> {
    r#type: &'static str,
    args: Vec<T>,
}

impl<T: MetadataRequest> BulkRequest<T> {
    pub fn new(args: Vec<T>) -> Self {
        BulkRequest {
            r#type: "bulk",
            args,
        }
    }
}
