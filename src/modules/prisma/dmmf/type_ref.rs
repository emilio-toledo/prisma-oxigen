use serde::{Deserialize, Serialize};

use super::field::Namespace;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ref<AllowedLocations> {
    pub is_list: bool,
    pub r#type: String,
    pub location: AllowedLocations,
    pub namespace: Option<Namespace>,
}
