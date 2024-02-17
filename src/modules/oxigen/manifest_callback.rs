use crate::modules::prisma::{GeneratorConfig, Request};

pub type ManifestCallback = Option<fn(ManifestCallbackParams)>;

pub struct ManifestCallbackParams<'a> {
    pub message: &'a Request,
    pub config: &'a GeneratorConfig,
}
