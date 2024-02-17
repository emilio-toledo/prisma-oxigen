use serde::{Deserialize, Serialize};

use super::{schema_arg::Arg, type_ref};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    name: String,
    constraints: TypeConstraints,
    meta: Option<TypeMeta>,
    fields: Vec<Arg>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ObjectTypes {
    model: Option<Vec<Type>>, // * for now there are no model InputTypes
    prisma: Vec<Type>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeConstraints {
    max_num_fields: Option<f64>,
    min_num_fields: Option<f64>,
    fields: Option<Vec<String>>,
}

pub type TypeRef = type_ref::Ref<TypeRefAllowedLocations>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TypeRefAllowedLocations {
    Scalar,
    InputObjectTypes,
    EnumTypes,
    FieldRefTypes,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeMeta {
    source: Option<String>,
}
