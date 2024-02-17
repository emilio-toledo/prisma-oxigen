use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryKey {
    name: Option<String>,
    fields: Vec<String>,
}
