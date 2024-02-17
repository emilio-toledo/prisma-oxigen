extern crate prisma_oxigen;

use prisma_oxigen::modules::oxigen::{generate_callback, generator::Generator, manifest_callback};

fn manifest(params: manifest_callback::Params) {
    println!("{:?}", params);
}

fn generate(params: generate_callback::Params) {
    println!("{:?}", params);
}

fn main() {
    Generator::new(Some(manifest), Some(generate));
}
