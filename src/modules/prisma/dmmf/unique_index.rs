use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    pub name: String,
    pub fields: Vec<String>,
}
