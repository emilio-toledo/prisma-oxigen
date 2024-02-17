use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Deprecation {
    pub since_version: String,
    pub reason: String,
    pub planned_removal_version: Option<String>,
}
