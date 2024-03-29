use crate::modules::prisma::{generator_config::Config, request::Request};

pub type Callback = Option<fn(Params)>;

#[derive(Debug)]
pub struct Params<'a> {
    pub message: &'a Request,
    pub config: &'a Config,
}
