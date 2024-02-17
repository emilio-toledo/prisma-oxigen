use serde::{Deserialize, Serialize};

use super::{field_default::Enum, field_kind::Kind};

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
    default: Option<Enum>,
    relation_from_fields: Option<Vec<String>>,
    relation_to_fields: Option<Vec<String>>,
    relation_on_delete: Option<String>,
    relation_name: Option<String>,
    documentation: Option<String>,
}
