use serde::{Deserialize, Serialize};

use super::InputObjectTypes;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    root_query_type: Option<String>,
    root_mutation_type: Option<String>,
    input_object_types: InputObjectTypes,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum FieldLocations {
    Scalar,
    InputObjectTypes,
    OutputObjectTypes,
    EnumTypes,
    FieldRefTypes,
}
