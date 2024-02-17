use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum FieldDefaultEnum {
    FieldDefault(FieldDefault),
    FieldDefaultScalar(FieldDefaultScalar),
    FieldDefaultScalarList(Vec<FieldDefaultScalar>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefault {
    name: String,
    args: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum FieldDefaultScalar {
    String(String),
    Boolean(bool),
    Number(f64),
}
