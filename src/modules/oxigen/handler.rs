use serde_json::json;
use std::io::stderr;

use super::{generate_callback, manifest_callback};
use crate::modules::prisma::{manifest::Manifest, request::Request, response::Response};

pub struct Handler {}
impl Handler {
    pub fn run(
        manifest_callback: manifest_callback::Callback,
        generate_callback: generate_callback::Callback,
    ) {
        loop {
            let message = Request::listen();

            match message.method.as_str() {
                "getManifest" => Self::on_manifest(&message, manifest_callback),
                "generate" => break Self::on_generate(&message, generate_callback),
                _ => (),
            };
        }
    }

    fn on_manifest(message: &Request, callback: manifest_callback::Callback) {
        let manifest = serde_json::to_value(&Manifest::default())
            .expect("Failed to parse manifest object as json");

        let response = Response {
            jsonrpc: message.jsonrpc.clone(),
            result: json!({"manifest": manifest }),
            id: message.id,
        };

        if let Some(callback) = callback {
            callback(manifest_callback::Params {
                message,
                config: &message.params.as_generator_config(),
            });
        }

        Response::send(&response, &mut stderr());
    }

    fn on_generate(message: &Request, callback: generate_callback::Callback) {
        if let Some(callback) = callback {
            callback(generate_callback::Params {
                message,
                options: &message.params.as_generator_options(),
            });
        }
    }
}
