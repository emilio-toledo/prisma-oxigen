use serde::{Deserialize, Serialize};

use super::type_ref;

pub type Ref = type_ref::Ref<AllowedLocations>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AllowedLocations {
    Scalar,
    InputObjectTypes,
    EnumTypes,
    FieldRefTypes,
}
