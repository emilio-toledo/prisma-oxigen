use serde::{Deserialize, Serialize};

use super::input_type::Type;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Types {
    model: Option<Vec<Type>>, // * for now there are no model InputTypes
    prisma: Vec<Type>,
}
