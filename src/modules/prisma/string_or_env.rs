use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StringOrEnv {
    pub from_env_var: Option<String>,
    pub value: Option<String>,
}
