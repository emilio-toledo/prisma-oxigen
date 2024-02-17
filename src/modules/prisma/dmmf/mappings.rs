use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Mappings {
    pub model_operations: Vec<ModelMapping>,
    pub other_operations: OtherOperations,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModelMapping {
    pub model: String,
    pub plural: Option<String>, // ! should be required according to Prisma's generator-helper definitions but I have seen it missing
    pub find_unique: Option<String>,
    pub find_unique_or_throw: Option<String>,
    pub find_first: Option<String>,
    pub find_first_or_throw: Option<String>,
    pub find_many: Option<String>,
    pub create: Option<String>,
    pub create_many: Option<String>,
    pub update: Option<String>,
    pub update_many: Option<String>,
    pub upsert: Option<String>,
    pub delete: Option<String>,
    pub delete_many: Option<String>,
    pub aggregate: Option<String>,
    pub group_by: Option<String>,
    pub count: Option<String>,
    pub find_raw: Option<String>,
    pub aggregate_raw: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OtherOperations {
    pub read: Vec<String>,
    pub write: Vec<String>,
}
