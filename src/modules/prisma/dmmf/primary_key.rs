use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    name: Option<String>,
    fields: Vec<String>,
}
