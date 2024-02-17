use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    kind: Kind,
    name: String,
    is_required: bool,
    is_list: bool,
    is_unique: bool,
    is_id: bool,
    is_read_only: bool,
    is_generated: Option<bool>, // * does not exist on 'type' but does on 'model'
    is_updated_at: Option<bool>, // * does not exist on 'type' but does on 'model'
    r#type: String,
    db_name: Option<String>,
    has_default_value: bool,
    default: Option<Default>,
    relation_from_fields: Option<Vec<String>>,
    relation_to_fields: Option<Vec<String>>,
    relation_on_delete: Option<String>,
    relation_name: Option<String>,
    documentation: Option<String>,
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
    name: String,
    args: Vec<Value>,
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
