use serde::{Deserialize, Serialize};

use super::{Field, PrimaryKey, UniqueIndex};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    name: String,
    db_name: Option<String>,
    fields: Vec<Field>,
    unique_fields: Vec<Vec<String>>,
    unique_indexes: Vec<UniqueIndex>,
    documentation: Option<String>,
    primary_key: Option<PrimaryKey>,
    is_generated: Option<bool>,
}
