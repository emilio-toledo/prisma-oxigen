use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{binary_target::Target, string_or_env::StringOrEnv};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub name: String,
    pub output: Option<StringOrEnv>,
    pub is_custom_output: Option<bool>,
    pub provider: StringOrEnv,
    pub config: HashMap<String, String>,
    pub binary_targets: Vec<Target>,
    pub preview_features: Vec<String>,
}
