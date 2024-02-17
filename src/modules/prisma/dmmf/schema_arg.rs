use serde::{Deserialize, Serialize};

use super::{Deprecation, InputTypeRef};
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaArg {
    name: String,
    comment: Option<String>,
    is_nullable: bool,
    is_required: bool,
    input_types: Vec<InputTypeRef>,
    deprecation: Option<Deprecation>,
}
