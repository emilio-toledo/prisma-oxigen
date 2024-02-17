use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataModelEnum {
    name: String,
    values: Vec<EnumValue>,
    db_name: Option<String>,
    documentation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    name: String,
    db_name: Option<String>,
}
