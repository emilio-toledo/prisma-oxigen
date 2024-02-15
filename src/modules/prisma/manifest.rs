use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub pretty_name: String,
    pub default_output: String,
    pub deny_list: Option<DenyList>,
    pub requires_generators: Option<Vec<String>>,
    pub required_engines: Option<Vec<Engine>>,
    pub requires_engines_version: Option<String>,
    pub version: String,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            pretty_name: "generator".to_string(),
            default_output: "./gen".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            deny_list: None,
            requires_generators: None,
            requires_engines_version: None,
            required_engines: None,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct DenyList {
    models: Option<Vec<String>>,
    fields: Option<Vec<String>>,
}

#[derive(Serialize, Debug)]
#[allow(dead_code)]
pub enum Engine {
    QueryEngine,
    LibQueryEngineNapi,
    MigrationEngine,
    IntrospectionEngine,
    PrismaFmt,
}
