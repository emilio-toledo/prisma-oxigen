use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FieldKind {
    Scalar,
    Object,
    Enum,
    Unsupported,
}
