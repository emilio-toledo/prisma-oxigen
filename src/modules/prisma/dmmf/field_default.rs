use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Enum {
    FieldDefault(Default),
    FieldDefaultScalar(DefaultScalar),
    FieldDefaultScalarList(Vec<DefaultScalar>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Default {
    name: String,
    args: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum DefaultScalar {
    String(String),
    Boolean(bool),
    Number(f64),
}
