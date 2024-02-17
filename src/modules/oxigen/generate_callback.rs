use crate::modules::prisma::{GeneratorOptions, Request};

pub type GenerateCallback = Option<fn(GenerateCallbackParams)>;

pub struct GenerateCallbackParams<'a> {
    pub message: &'a Request,
    pub options: &'a GeneratorOptions,
}
