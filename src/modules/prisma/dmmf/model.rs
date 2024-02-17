use serde::{Deserialize, Serialize};

use super::{field::Field, primary_key::Key, unique_index::Index};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    name: String,
    db_name: Option<String>,
    fields: Vec<Field>,
    unique_fields: Vec<Vec<String>>,
    unique_indexes: Vec<Index>,
    documentation: Option<String>,
    primary_key: Option<Key>,
    is_generated: Option<bool>,
}
