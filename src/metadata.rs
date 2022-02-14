use serde::{Deserialize, Serialize};

use crate::types::{
    CreateArrayRelationship, CreateObjectRelationship, CreateRelationship, RelType,
    SQLFKRelationships,
};

#[derive(Deserialize, Debug)]
pub struct Metadata {
    version: u8,
    sources: Vec<MetadataSource>,
}

impl Metadata {
    pub fn get_untracked_tables(&self, tables: Vec<QualifiedTable>) -> Vec<QualifiedTable> {
        let mut result = vec![];
        for table in tables {
            if !self.is_table_tracked(&table) {
                result.push(table)
            }
        }
        result
    }
    pub fn get_untracked_relationships<'a>(
        &'a self,
        relationships: &'a Vec<SQLFKRelationships>,
        source: &'a str,
    ) -> Vec<CreateRelationship> {
        let mut array_rels: Vec<CreateArrayRelationship> = vec![];
        let mut object_rels: Vec<CreateObjectRelationship> = vec![];
        for rel in relationships {
            let (obj_rel, arr_rel) = rel.get_relationships(source);
            if !self.is_relationship_tracked(&rel, RelType::Array, source) {
                array_rels.push(arr_rel);
            }
            if !self.is_relationship_tracked(&rel, RelType::Object, source) {
                object_rels.push(obj_rel);
            }
        }

        array_rels
            .into_iter()
            .map(|a| CreateRelationship::Array(a))
            .chain(
                object_rels
                    .into_iter()
                    .map(|o| CreateRelationship::Object(o)),
            )
            .collect()
    }
    pub fn is_table_tracked(&self, table: &QualifiedTable) -> bool {
        self.sources
            .iter()
            .any(|ms| ms.tables.iter().any(|te| &te.table == table))
    }
    pub fn is_relationship_tracked(
        &self,
        relationship: &SQLFKRelationships,
        rel_type: RelType,
        source: &str,
    ) -> bool {
        let arr_table = QualifiedTable {
            name: relationship.ref_table_name.clone(),
            schema: relationship.ref_table_schema.clone(),
        };
        let obj_table = QualifiedTable {
            name: relationship.table_name.clone(),
            schema: relationship.table_schema.clone(),
        };
        let rel_table = match rel_type {
            RelType::Array => &arr_table,
            RelType::Object => &obj_table,
        };
        self.sources
            .iter()
            .find(|&s| s.name == source)
            .map(|source| source.tables.iter().find(|&te| te.table == *rel_table))
            .flatten()
            .map(|te| {
                &te.table == rel_table && {
                    let rels = match &rel_type {
                        RelType::Array => te
                            .array_relationships
                            .iter()
                            .any(|rel| rel.using.foreign_key_constraint_on.table == obj_table),
                        RelType::Object => te.object_relationships.iter().any(|rel| {
                            relationship
                                .column_mapping
                                .contains_key(&rel.using.foreign_key_constraint_on)
                        }),
                    };
                    rels
                }
            })
            .unwrap_or_default()
    }
}

#[derive(Deserialize, Debug)]
struct MetadataSource {
    name: String,
    kind: Option<String>,
    tables: Vec<TableEntry>,
    functions: Option<Vec<FunctionEntry>>,
}

#[derive(Deserialize, Debug)]
struct TableEntry {
    table: QualifiedTable,
    object_relationships: Vec<ObjectRelationships>,
    array_relationships: Vec<ArrayRelationships>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
pub struct QualifiedTable {
    pub(crate) name: String,
    pub(crate) schema: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ObjectRelationships {
    pub(crate) name: String,
    pub(crate) using: ObjRelUsing,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ObjRelUsing {
    pub(crate) foreign_key_constraint_on: String, // no need to support manual configuration yet
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ArrayRelUsing {
    pub(crate) foreign_key_constraint_on: ArrayRelUsingFKeyOn, // no need to support manual configuration yet
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct ArrayRelUsingFKeyOn {
    pub(crate) column: String,
    pub(crate) table: QualifiedTable,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ArrayRelationships {
    pub(crate) name: String,
    pub(crate) using: ArrayRelUsing,
}

type QualifiedFunction = QualifiedTable;

#[derive(Deserialize, Debug)]
struct FunctionEntry {
    function: QualifiedFunction,
}
