use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StringOrEnv {
    pub from_env_var: Option<String>,
    pub value: Option<String>,
}
