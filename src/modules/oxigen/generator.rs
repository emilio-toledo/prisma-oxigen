use super::{
    handler::{GenerateCallback, ManifestCallback},
    Handler,
};

pub struct Generator {}
impl Generator {
    pub fn new(manifest_callback: GenerateCallback, generate_callback: ManifestCallback) {
        Handler::run(manifest_callback, generate_callback);
    }
}
