use serde::{Deserialize, Serialize};

use super::{deprecation::Deprecation, input_type_ref::Ref};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Arg {
    name: String,
    comment: Option<String>,
    is_nullable: bool,
    is_required: bool,
    input_types: Vec<Ref>,
    deprecation: Option<Deprecation>,
}
