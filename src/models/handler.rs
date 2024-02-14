use super::prisma::{Manifest, Request, Response};
use serde_json::json;

pub type OnManifestCallback = Option<fn(OnManifestCallbackParams)>;
pub type OnGenerateCallback = Option<fn(OnGenerateCallbackParams)>;

pub type OnManifestCallbackParams<'a> = (&'a Request, i32);
pub type OnGenerateCallbackParams<'a> = (&'a Request, i32);

pub struct Handler {}
impl Handler {
    pub fn run(manifest_callback: OnManifestCallback, generate_callback: OnGenerateCallback) {
        loop {
            let message = Request::listen();

            println!("{:?}", message);

            match message.method.as_str() {
                "getManifest" => Self::on_manifest(&message, manifest_callback),
                "generate" => break Self::on_generate(&message, generate_callback),
                _ => (),
            };
        }
    }

    fn on_manifest(message: &Request, callback: OnManifestCallback) {
        let manifest = serde_json::to_value(&Manifest::default())
            .expect("Failed to parse manifest object as json");

        let response = Response {
            jsonrpc: message.jsonrpc.clone(),
            result: json!({"manifest": manifest }),
            id: message.id,
        };

        if let Some(callback) = callback {
            callback((&message, 32));
        }

        Response::send(response);
    }

    fn on_generate(message: &Request, callback: OnGenerateCallback) {
        if let Some(callback) = callback {
            callback((&message, 32));
        }
    }
}
