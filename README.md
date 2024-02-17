# prisma-oxigen

prisma-oxigen is a comprehensive Rust package designed to seamlessly implement Prisma's JSONRPC Interface along with its types, providing essential helper functions to create Prisma generators effortlessly.

### Key Features:

- **Full Implementation** of Prisma's JSONRPC Interface, ensuring seamless communication with Prisma engines.
- **Extensive Type Support**, encapsulating Prisma's model and field types within Rust's strong type system, leading to safer and more predictable database interactions.
- **Helper Functions** to streamline the creation of Prisma generators, making it easier to extend Prisma's capabilities or integrate with other tools and frameworks.

## Getting Started

To use prisma-oxigen in your project, you will need to have Rust and Cargo installed on your system. If you don't have them installed, visit [rust-lang.org](https://rust-lang.org) to get started.

### Installation

Add prisma-oxigen to your `Cargo.toml`:

```toml
[dependencies]
prisma-oxigen = "1.0.0"
```

Then, run `cargo build` to download and compile the package along with its dependencies.

### Basic Usage

prisma-oxigen allows you to extend Prisma by writing your custom generators. Generators are used to transform the Prisma schema into code or other outputs. Here's a skeleton to start writing a custom generator:

```rust
use prisma_oxigen::modules::oxigen::{generate_callback, generator::Generator, manifest_callback};

fn manifest(params: manifest_callback::Params) {
}

fn generate(params: generate_callback::Params) {
}

fn main() {
    Generator::new(Some(manifest), Some(generate));
}
```

## Documentation

For more detailed information on the API and advanced usage, refer to the prisma-oxigen [documentation page](#).

## Contributing

Contributions are welcome! Whether it's submitting a bug report, a feature request, or a pull request, all forms of contributions help improve prisma-oxigen.

## License

prisma-oxigen is distributed under the terms of the Apache 2.0 license. See [LICENSE](#) for details.