use serde::{Deserialize, Serialize};

use super::{
    deprecation::Deprecation,
    input::{ObjectTypes, TypeRef},
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    root_query_type: Option<String>,
    root_mutation_type: Option<String>,
    input_object_types: ObjectTypes,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Arg {
    name: String,
    comment: Option<String>,
    is_nullable: bool,
    is_required: bool,
    input_types: Vec<TypeRef>,
    deprecation: Option<Deprecation>,
}
