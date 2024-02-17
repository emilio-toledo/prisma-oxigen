use super::{GenerateCallback, Handler, ManifestCallback};

pub struct Generator {}
impl Generator {
    pub fn new(manifest_callback: ManifestCallback, generate_callback: GenerateCallback) {
        Handler::run(manifest_callback, generate_callback);
    }
}
