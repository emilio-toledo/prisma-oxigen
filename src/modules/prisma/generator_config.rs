use super::{BinaryTarget, StringOrEnv};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorConfig {
    pub name: String,
    pub output: Option<StringOrEnv>,
    pub is_custom_output: Option<bool>,
    pub provider: StringOrEnv,
    pub config: HashMap<String, String>,
    pub binary_targets: Vec<BinaryTarget>,
    pub preview_features: Vec<String>,
}
