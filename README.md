# prisma-oxigen

Prisma Oxidized Generator (Oxigen) is a crate designed to simplify the creation of Prisma generators. It aims to provide essential helper functions and utilities similar to [@prisma/generator-helper](https://www.npmjs.com/package/@prisma/generator-helper) but for Rust.

### Key Features:

- **Full Implementation** of Prisma's JSONRPC Interface, ensuring seamless communication with Prisma engines.
- **Extensive Type Support**, encapsulating Prisma's model and field types within Rust's strong type system.
- **Helper Functions** to streamline the creation of Prisma generators, making it easier to extend Prisma's capabilities or integrate with other tools and frameworks.

## Getting Started

To use Oxigen in your project, you will need to have Rust and Cargo installed on your system. If you don't have them installed, visit [rust-lang.org](https://rust-lang.org) to get started.

### Installation

Add prisma-oxigen to your `Cargo.toml`:

```toml
[dependencies]
prisma-oxigen = "1.0.0"
```

Then, run `cargo build` to download and compile the package along with its dependencies.

### Basic Usage

Oxigen allows you to extend Prisma by writing your custom generators. Generators are used to transform the Prisma schema into code or other outputs. Here's a skeleton to start writing a custom generator:

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

For more detailed information on the API and advanced usage, refer to the prisma-oxigen [documentation page](https://docs.rs/prisma-oxigen/latest/prisma_oxigen/).

## Contributing

Contributions are welcome! Whether it's submitting a bug report, a feature request, or a pull request, all forms of contributions help improve Oxigen.

## License

prisma-oxigen is distributed under the terms of the Apache 2.0 license. See [LICENSE](https://github.com/emilio-toledo/prisma-oxigen/blob/main/LICENSE.md) for details.

## Special Thanks
I want to thank everyone who has worked on the following projects, as without them Oxigen wouldn't exist:

- The author of [prismaio.notion.site/prisma-generators](https://prismaio.notion.site/Prisma-Generators-a2cdf262207a4e9dbcd0e362dfac8dc0).
- [Prisma Rust Client](https://github.com/Brendonovich/prisma-client-rust).
- [The Prisma Generator Helper](https://github.com/prisma/prisma/tree/main/packages/generator-helper) source code (aka the Prisma team).
