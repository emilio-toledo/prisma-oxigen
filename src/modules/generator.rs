use super::{
    handler::{OnGenerateCallback, OnManifestCallback},
    Handler,
};

pub struct Generator {}
impl Generator {
    pub fn new(manifest_callback: OnGenerateCallback, generate_callback: OnManifestCallback) {
        Handler::run(manifest_callback, generate_callback);
    }
}
