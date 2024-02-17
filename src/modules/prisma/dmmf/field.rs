use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub kind: Kind,
    pub name: String,
    pub is_required: bool,
    pub is_list: bool,
    pub is_unique: bool,
    pub is_id: bool,
    pub is_read_only: bool,
    pub is_generated: Option<bool>, // * does not exist on 'type' but does on 'model'
    pub is_updated_at: Option<bool>, // * does not exist on 'type' but does on 'model'
    pub r#type: String,
    pub db_name: Option<String>,
    pub has_default_value: bool,
    pub default: Option<Default>,
    pub relation_from_fields: Option<Vec<String>>,
    pub relation_to_fields: Option<Vec<String>>,
    pub relation_on_delete: Option<String>,
    pub relation_name: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Kind {
    Scalar,
    Object,
    Enum,
    Unsupported,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Default {
    FieldDefault(FieldDefault),
    FieldDefaultScalar(FieldDefaultScalar),
    FieldDefaultScalarList(Vec<FieldDefaultScalar>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefault {
    pub name: String,
    pub args: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum FieldDefaultScalar {
    String(String),
    Boolean(bool),
    Number(f64),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Namespace {
    Model,
    Prisma,
}
