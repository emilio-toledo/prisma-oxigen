use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub from_env_var: Option<String>,
    pub value: Option<String>,
    pub native: bool,
}
