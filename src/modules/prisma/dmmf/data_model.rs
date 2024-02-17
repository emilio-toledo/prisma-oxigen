use serde::{Deserialize, Serialize};

use super::{data_model_enum::Enum, model};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    enums: Vec<Enum>,
    models: Vec<model::Model>,
    types: Vec<model::Model>,
}
