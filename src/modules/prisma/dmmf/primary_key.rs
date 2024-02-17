use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    pub name: Option<String>,
    pub fields: Vec<String>,
}
