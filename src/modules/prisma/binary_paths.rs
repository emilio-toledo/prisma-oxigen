use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Paths {
    pub schema_engine: Option<HashMap<String, String>>,
    pub query_engine: Option<HashMap<String, String>>,
    pub libquery_engine: Option<HashMap<String, String>>,
}
