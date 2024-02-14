use std::io::{stderr, stdin, Write};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub params: Value,
    pub id: i32,
}

impl Request {
    pub fn listen() -> Request {
        let mut message = String::new();

        stdin()
            .read_line(&mut message)
            .expect("failed to read the jsonrpc message from the prisma engine");

        serde_json::from_str(&message).unwrap()
    }
}

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
