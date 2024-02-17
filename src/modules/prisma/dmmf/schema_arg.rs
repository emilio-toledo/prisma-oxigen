use serde::{Deserialize, Serialize};

use super::{deprecation::Deprecation, input::TypeRef};

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
