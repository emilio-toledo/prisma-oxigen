use serde::{Deserialize, Serialize};

use super::{input_type_constraints::Constraints, input_type_meta::Meta, schema_arg::Arg};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    name: String,
    constraints: Constraints,
    meta: Option<Meta>,
    fields: Vec<Arg>,
}
