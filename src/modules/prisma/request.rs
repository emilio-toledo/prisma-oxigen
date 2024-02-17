use serde::{Deserialize, Serialize};
use std::io::stdin;

use super::{GeneratorConfig, GeneratorOptions};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub params: Params,
    pub id: f64,
}

impl Request {
    pub fn listen() -> Request {
        let mut message = String::new();

        stdin()
            .read_line(&mut message)
            .expect("Failed to read the jsonrpc message from the prisma engine");

        serde_json::from_str(&message).expect("Failed to parse jsonrpc message")
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Params {
    GeneratorConfig(GeneratorConfig),
    GeneratorOptions(GeneratorOptions),
}
