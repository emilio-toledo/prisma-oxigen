use crate::modules::prisma::{generator_options::Options, request::Request};

pub type Callback = Option<fn(Params)>;

#[derive(Debug)]
pub struct Params<'a> {
    pub message: &'a Request,
    pub options: &'a Options,
}
