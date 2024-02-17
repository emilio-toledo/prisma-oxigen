use serde::{Deserialize, Serialize};

use super::{
    deprecation::Deprecation,
    input::{ObjectTypes, TypeRef},
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub root_query_type: Option<String>,
    pub root_mutation_type: Option<String>,
    pub input_object_types: ObjectTypes,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Arg {
    pub name: String,
    pub comment: Option<String>,
    pub is_nullable: bool,
    pub is_required: bool,
    pub input_types: Vec<TypeRef>,
    pub deprecation: Option<Deprecation>,
}
