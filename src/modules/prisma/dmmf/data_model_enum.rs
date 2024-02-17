use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Enum {
    name: String,
    values: Vec<Value>,
    db_name: Option<String>,
    documentation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    name: String,
    db_name: Option<String>,
}
