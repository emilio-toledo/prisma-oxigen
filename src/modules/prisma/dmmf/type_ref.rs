use serde::{Deserialize, Serialize};

use super::field::Namespace;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ref<AllowedLocations> {
    is_list: bool,
    r#type: String,
    location: AllowedLocations,
    namespace: Option<Namespace>,
}
