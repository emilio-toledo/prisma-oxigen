use super::{BinaryPaths, DataSource, Document, GeneratorConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorOptions {
    pub datamodel: String, // ! datamodel could be renamed to schema in the future
    pub datasources: Vec<DataSource>,
    pub generator: GeneratorConfig,
    pub binary_paths: Option<BinaryPaths>,
    pub dmmf: Document,
    pub other_generators: Vec<GeneratorConfig>,
    pub schema_path: String,
    pub version: String,
    pub postinstall: Option<bool>,
    pub no_engine: Option<bool>,
}
