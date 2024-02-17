use serde::{Deserialize, Serialize};

use super::{field::Field, primary_key::Key, unique_index::Index};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataModel {
    pub enums: Vec<Enum>,
    pub models: Vec<Model>,
    pub types: Vec<Model>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub name: String,
    pub db_name: Option<String>,
    pub fields: Vec<Field>,
    pub unique_fields: Vec<Vec<String>>,
    pub unique_indexes: Vec<Index>,
    pub documentation: Option<String>,
    pub primary_key: Option<Key>,
    pub is_generated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Enum {
    pub name: String,
    pub values: Vec<EnumValue>,
    pub db_name: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub name: String,
    pub db_name: Option<String>,
}
