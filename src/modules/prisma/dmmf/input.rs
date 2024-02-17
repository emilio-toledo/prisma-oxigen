use serde::{Deserialize, Serialize};

use super::{schema::Arg, type_ref};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub name: String,
    pub constraints: TypeConstraints,
    pub meta: Option<TypeMeta>,
    pub fields: Vec<Arg>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ObjectTypes {
    pub model: Option<Vec<Type>>, // * for now there are no model InputTypes
    pub prisma: Vec<Type>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeConstraints {
    pub max_num_fields: Option<f64>,
    pub min_num_fields: Option<f64>,
    pub fields: Option<Vec<String>>,
}

pub type TypeRef = type_ref::Ref<TypeRefAllowedLocations>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TypeRefAllowedLocations {
    Scalar,
    InputObjectTypes,
    OutputObjectTypes,
    EnumTypes,
    FieldRefTypes,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeMeta {
    pub source: Option<String>,
}
