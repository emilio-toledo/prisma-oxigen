use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Provider, StringOrEnv};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataSource {
    pub name: String,
    pub provider: Provider,
    pub active_provider: Provider,
    pub url: StringOrEnv,
    pub direct_url: Option<StringOrEnv>,
    pub schemas: Vec<Value>,
}
