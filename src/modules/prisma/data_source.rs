use serde::Deserialize;
use serde_json::Value;

use super::{Connector, StringOrEnv};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataSource {
    pub name: String,
    pub provider: Connector,
    pub active_provider: Connector,
    pub url: StringOrEnv,
    pub direct_url: Option<StringOrEnv>,
    pub schemas: Vec<Value>,
}