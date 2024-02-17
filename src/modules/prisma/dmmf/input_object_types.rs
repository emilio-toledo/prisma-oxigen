use serde::{Deserialize, Serialize};

use super::InputType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InputObjectTypes {
    model: Option<Vec<InputType>>, // * for now there are no model InputTypes
    prisma: Vec<InputType>,
}
