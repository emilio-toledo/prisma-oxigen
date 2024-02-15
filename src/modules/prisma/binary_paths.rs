use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct BinaryPaths {
    pub schema_engine: Option<HashMap<String, String>>,
    pub query_engine: Option<HashMap<String, String>>,
    pub libquery_engine: Option<HashMap<String, String>>,
}
