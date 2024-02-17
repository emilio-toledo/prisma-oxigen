use crate::modules::prisma::{generator_options::Options, request::Request};

pub type Callback = Option<fn(Params)>;

pub struct Params<'a> {
    pub message: &'a Request,
    pub options: &'a Options,
}
