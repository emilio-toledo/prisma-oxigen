use serde::{Deserialize, Serialize};

use super::{DataModelEnum, Model};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataModel {
    enums: Vec<DataModelEnum>,
    models: Vec<Model>,
    types: Vec<Model>,
}
