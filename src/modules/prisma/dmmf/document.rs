use serde::{Deserialize, Serialize};

use super::{data_model::Model, mappings::Mappings, schema::Schema};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    datamodel: Model,
    schema: Schema,
    mappings: Mappings,
}
