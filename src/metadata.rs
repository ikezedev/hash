use serde::{Deserialize, Serialize};

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
    pub fn is_table_tracked(&self, table: &QualifiedTable) -> bool {
        self.sources
            .iter()
            .any(|ms| ms.tables.iter().any(|te| &te.table == table))
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
    name: String,
    schema: String,
}

#[derive(Deserialize, Debug)]
struct ObjectRelationships {
    name: String,
    using: ObjRelUsing,
}

#[derive(Deserialize, Debug)]
struct ObjRelUsing {
    foreign_key_constraint_on: String, // no need to support manual configuration yet
}

#[derive(Deserialize, Debug)]
struct ArrayRelUsing {
    foreign_key_constraint_on: ArrayRelUsingFKeyOn, // no need to support manual configuration yet
}

#[derive(Deserialize, Debug)]
struct ArrayRelUsingFKeyOn {
    column: String,
    table: TableName,
}

#[derive(Deserialize, Debug)]
enum TableName {
    String,
    QualifiedTable,
}

#[derive(Deserialize, Debug)]
struct ArrayRelationships {
    name: String,
    using: ArrayRelUsing,
}

type QualifiedFunction = QualifiedTable;

#[derive(Deserialize, Debug)]
struct FunctionEntry {
    function: QualifiedFunction,
}
