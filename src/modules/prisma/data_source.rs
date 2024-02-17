use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{provider::Provider, string_or_env::StringOrEnv};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub name: String,
    pub provider: Provider,
    pub active_provider: Provider,
    pub url: StringOrEnv,
    pub direct_url: Option<StringOrEnv>,
    pub schemas: Vec<Value>,
}
