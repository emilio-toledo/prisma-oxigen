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
    pub fn send<'a, Writer: Write>(response: &'a Response, writer: &'a mut Writer) {
        let mut payload = serde_json::to_vec(&response).expect("Failed to serialize response");

        payload.push(b'\n');

        writer.write(&payload).expect("Failed to write to stderr");
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

    use std::io::Cursor;

    use super::Response;

    #[test]
    fn send() {
        let response = Response::default();

        let mut expected = serde_json::to_vec(&response).expect("Failed to serialize response");
        expected.push(b'\n');

        let mut writer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        Response::send(&response, &mut writer);

        let actual = writer.into_inner();

        assert_eq!(expected, actual);
    }
}
