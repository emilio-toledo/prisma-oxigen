use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Mappings {
    model_operations: Vec<ModelMapping>,
    other_operations: OtherOperations,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModelMapping {
    model: String,
    plural: Option<String>, // ! should be required according to Prisma's generator-helper definitions but I have seen it missing
    find_unique: Option<String>,
    find_unique_or_throw: Option<String>,
    find_first: Option<String>,
    find_first_or_throw: Option<String>,
    find_many: Option<String>,
    create: Option<String>,
    create_many: Option<String>,
    update: Option<String>,
    update_many: Option<String>,
    upsert: Option<String>,
    delete: Option<String>,
    delete_many: Option<String>,
    aggregate: Option<String>,
    group_by: Option<String>,
    count: Option<String>,
    find_raw: Option<String>,
    aggregate_raw: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OtherOperations {
    read: Vec<String>,
    write: Vec<String>,
}
