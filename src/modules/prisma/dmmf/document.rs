use serde::{Deserialize, Serialize};

use super::{data_model::DataModel, mappings::Mappings, schema::Schema};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub datamodel: DataModel,
    pub schema: Schema,
    pub mappings: Mappings,
}
