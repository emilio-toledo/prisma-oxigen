use serde::{Deserialize, Serialize};

use super::FieldNamespace;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeRef<AllowedLocations> {
    is_list: bool,
    r#type: String,
    location: AllowedLocations,
    namespace: Option<FieldNamespace>,
}
