use serde::{Deserialize, Serialize};

use super::{DataModel, Mappings, Schema};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    datamodel: DataModel,
    schema: Schema,
    mappings: Mappings,
}
