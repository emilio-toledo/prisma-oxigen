use serde::{Deserialize, Serialize};

use super::{
    binary_paths::Paths, data_source::Source, dmmf::document::Document, generator_config::Config,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub datamodel: String, // ! datamodel could be renamed to schema in the future
    pub datasources: Vec<Source>,
    pub generator: Config,
    pub binary_paths: Option<Paths>,
    pub dmmf: Document,
    pub other_generators: Vec<Config>,
    pub schema_path: String,
    pub version: String,
    pub postinstall: Option<bool>,
    pub no_engine: Option<bool>,
}
