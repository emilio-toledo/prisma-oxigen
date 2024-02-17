use crate::modules::prisma::{GeneratorConfig, GeneratorOptions, Manifest, Request, Response};
use serde_json::json;
use std::io::stderr;

pub type ManifestCallback = Option<fn(ManifestCallbackParams)>;
pub type GenerateCallback = Option<fn(GenerateCallbackParams)>;

pub struct ManifestCallbackParams<'a> {
    pub message: &'a Request,
    pub config: &'a GeneratorConfig,
}

pub struct GenerateCallbackParams<'a> {
    pub message: &'a Request,
    pub options: &'a GeneratorOptions,
}

pub struct Handler {}
impl Handler {
    pub fn run(manifest_callback: ManifestCallback, generate_callback: GenerateCallback) {
        loop {
            let message = Request::listen();

            match message.method.as_str() {
                "getManifest" => Self::on_manifest(&message, manifest_callback),
                "generate" => break Self::on_generate(&message, generate_callback),
                _ => (),
            };
        }
    }

    fn on_manifest(message: &Request, callback: ManifestCallback) {
        let manifest = serde_json::to_value(&Manifest::default())
            .expect("Failed to parse manifest object as json");

        let response = Response {
            jsonrpc: message.jsonrpc.clone(),
            result: json!({"manifest": manifest }),
            id: message.id,
        };

        if let Some(callback) = callback {
            callback(ManifestCallbackParams {
                message,
                config: &message.params.as_generator_config(),
            });
        }

        Response::send(&response, &mut stderr());
    }

    fn on_generate(message: &Request, callback: GenerateCallback) {
        if let Some(callback) = callback {
            callback(GenerateCallbackParams {
                message,
                options: &message.params.as_generator_options(),
            });
        }
    }
}
