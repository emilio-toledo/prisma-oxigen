use super::{generate_callback, handler::Handler, manifest_callback};

pub struct Generator {}
impl Generator {
    pub fn new(
        manifest_callback: manifest_callback::Callback,
        generate_callback: generate_callback::Callback,
    ) {
        Handler::run(manifest_callback, generate_callback);
    }
}
