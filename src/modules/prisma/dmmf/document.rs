use serde::{Deserialize, Serialize};

use super::{data_model::DataModel, mappings::Mappings, schema::Schema};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    datamodel: DataModel,
    schema: Schema,
    mappings: Mappings,
}
