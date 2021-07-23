# machinery-rs

Rust bindings for [The Machinery](https://ourmachinery.com/) game engine.

The repository hosts the following parts:

- [![Crates.io](https://img.shields.io/crates/v/machinery.svg?label=machinery)](https://crates.io/crates/machinery) [![docs.rs](https://docs.rs/machinery/badge.svg)](https://docs.rs/machinery/) - Generated bindings and additional utilities for creating plugins.
- [![Crates.io](https://img.shields.io/crates/v/machinery-macros.svg?label=machinery-macros)](https://crates.io/crates/machinery-macros) [![docs.rs](https://docs.rs/machinery-macros/badge.svg)](https://docs.rs/machinery-macros/) - Procedural macro utilities for creating plugins.
- `machinery-generator` - Generator for generating Rust bindings from headers.
- `example` - Short example plugin showcasing the bindings.

## Building Example

The included example depends on [cargo-make](https://github.com/sagiegurari/cargo-make) to copy the
plugin to the plugins directory automatically.

```
cd example
cargo make machinery
```

## Generator

This project includes a generator to generate bindings and utilities from The Machinery and plugins'
headers as long as they conform to The Machinery header guidelines.
You can also use this generator for your own third party projects.
Generation can be configured through `Machinery.toml`.

```
cargo make generator
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License (Expat) ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
