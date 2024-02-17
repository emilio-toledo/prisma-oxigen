use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Deprecation {
    since_version: String,
    reason: String,
    planned_removal_version: Option<String>,
}
