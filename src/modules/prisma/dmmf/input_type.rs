use serde::{Deserialize, Serialize};

use super::{InputTypeConstraints, InputTypeMeta, SchemaArg};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InputType {
    name: String,
    constraints: InputTypeConstraints,
    meta: Option<InputTypeMeta>,
    fields: Vec<SchemaArg>,
}
