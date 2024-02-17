use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Constraints {
    max_num_fields: Option<f64>,
    min_num_fields: Option<f64>,
    fields: Option<Vec<String>>,
}
