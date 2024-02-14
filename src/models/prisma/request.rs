use serde::Deserialize;
use serde_json::Value;
use std::io::stdin;

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
