use super::{BinaryPaths, DataSource, GeneratorConfig};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorOptions {
    pub datamodel: String,
    pub datasources: Vec<DataSource>,
    pub generator: GeneratorConfig,
    pub binary_paths: Option<BinaryPaths>,
    pub dmmf: Value,
    pub other_generators: Value,
    pub schema_path: String,
    pub version: String,
    pub postinstall: Option<bool>,
    pub no_engine: Option<bool>,
}
