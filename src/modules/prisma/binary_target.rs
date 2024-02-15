use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BinaryTarget {
    pub from_env_var: Option<String>,
    pub value: Option<String>,
    pub native: bool,
}
