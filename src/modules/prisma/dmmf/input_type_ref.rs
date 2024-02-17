use serde::{Deserialize, Serialize};

use super::type_ref;

pub type Ref = type_ref::Ref<InputTypeRefAllowedLocations>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum InputTypeRefAllowedLocations {
    Scalar,
    InputObjectTypes,
    EnumTypes,
    FieldRefTypes,
}
