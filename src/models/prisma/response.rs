use serde::Serialize;
use serde_json::Value;
use std::io::{stderr, Write};

#[derive(Serialize, Debug)]
pub struct Response {
    pub jsonrpc: String,
    pub result: Value,
    pub id: i32,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Value::Null,
            id: 1,
        }
    }
}

impl Response {
    pub fn send(response: Response) {
        let mut payload =
            serde_json::to_vec(&response).expect("Failed to transform response to vector");

        payload.push(b'\n');

        stderr()
            .write(&payload)
            .expect("Failed to write to prisma engine");
    }
}
