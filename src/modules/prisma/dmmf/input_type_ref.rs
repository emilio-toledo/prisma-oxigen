use super::TypeRef;
use serde::{Deserialize, Serialize};

pub type InputTypeRef = TypeRef<InputTypeRefAllowedLocations>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum InputTypeRefAllowedLocations {
    Scalar,
    InputObjectTypes,
    EnumTypes,
    FieldRefTypes,
}
