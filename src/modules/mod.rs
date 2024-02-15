mod generator;
mod handler;

pub mod prisma {
    mod binary_paths;
    mod binary_target;
    mod connector;
    mod data_source;
    mod generator_config;
    mod generator_options;
    mod manifest;
    mod request;
    mod response;
    mod string_or_env;

    pub use binary_paths::*;
    pub use binary_target::*;
    pub use connector::*;
    pub use data_source::*;
    pub use generator_config::*;
    pub use generator_options::*;
    pub use manifest::*;
    pub use request::*;
    pub use response::*;
    pub use string_or_env::*;
}

pub use generator::*;
pub use handler::*;
