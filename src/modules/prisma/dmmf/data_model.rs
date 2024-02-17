use serde::{Deserialize, Serialize};

use super::{field::Field, primary_key::Key, unique_index::Index};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataModel {
    enums: Vec<Enum>,
    models: Vec<Model>,
    types: Vec<Model>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    name: String,
    db_name: Option<String>,
    fields: Vec<Field>,
    unique_fields: Vec<Vec<String>>,
    unique_indexes: Vec<Index>,
    documentation: Option<String>,
    primary_key: Option<Key>,
    is_generated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Enum {
    name: String,
    values: Vec<EnumValue>,
    db_name: Option<String>,
    documentation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    name: String,
    db_name: Option<String>,
}
