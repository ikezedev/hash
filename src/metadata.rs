use serde::Deserialize;

#[derive(Deserialize)]
pub struct Metadata {
    version: u8,
    sources: Vec<MetadataSource>,
}

#[derive(Deserialize)]
struct MetadataSource {
    name: String,
    kind: Option<String>,
    tables: Vec<TableEntry>,
    functions: Option<Vec<FunctionEntry>>,
}

#[derive(Deserialize)]
struct TableEntry {
    table: QualifiedTable,
    object_relationships: Vec<ObjectRelationships>,
    array_relationships: Vec<ArrayRelationships>,
}

#[derive(Deserialize)]
struct QualifiedTable {
    name: String,
    schema: String,
}

#[derive(Deserialize)]
struct ObjectRelationships {
    name: String,
    using: ObjRelUsing,
}

#[derive(Deserialize)]
struct ObjRelUsing {
    foreign_key_constraint_on: String, // no need to support manual configuration yet
}

#[derive(Deserialize)]
struct ArrayRelUsing {
    foreign_key_constraint_on: ArrayRelUsingFKeyOn, // no need to support manual configuration yet
}

#[derive(Deserialize)]
struct ArrayRelUsingFKeyOn {
    column: String,
    table: TableName,
}

#[derive(Deserialize)]
enum TableName {
    String,
    QualifiedTable,
}

#[derive(Deserialize)]
struct ArrayRelationships {
    name: String,
    using: ArrayRelUsing,
}

type QualifiedFunction = QualifiedTable;

#[derive(Deserialize)]
struct FunctionEntry {
    function: QualifiedFunction,
}
