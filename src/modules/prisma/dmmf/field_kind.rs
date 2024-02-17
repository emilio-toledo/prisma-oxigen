use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Kind {
    Scalar,
    Object,
    Enum,
    Unsupported,
}
