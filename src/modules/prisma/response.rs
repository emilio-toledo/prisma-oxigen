use serde::Serialize;
use serde_json::Value;
use std::io::Write;

#[derive(Serialize, Debug)]
pub struct Response {
    pub jsonrpc: String,
    pub result: Value,
    pub id: i32,
}

impl Response {
    pub fn send<Writer: Write>(response: Response, writer: &mut Writer) -> &mut Writer {
        let mut payload = serde_json::to_vec(&response).expect("Failed to serialize response");

        payload.push(b'\n');

        writer.write(&payload).expect("Failed to write to stderr");

        writer
    }
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

#[cfg(test)]
mod tests {
    use super::Response;

    #[test]
    fn send() {
        let response = Response::default();

        let mut mock = serde_json::to_vec(&response).expect("Failed to serialize response");
        mock.push(b'\n');

        assert_eq!(mock, Response::send(response, &mut Vec::new()).clone());
    }
}
