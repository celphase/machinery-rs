# machinery-rs

Rust bindings for [The Machinery](https://ourmachinery.com/) game engine.

The repository hosts the following parts:

- [![Crates.io](https://img.shields.io/crates/v/machinery.svg?label=machinery)](https://crates.io/crates/machinery) [![docs.rs](https://docs.rs/machinery/badge.svg)](https://docs.rs/machinery/) - Utilities and safe wrappers for working with The Machinery game engine.
- [![Crates.io](https://img.shields.io/crates/v/machinery-api.svg?label=machinery-api)](https://crates.io/crates/machinery) [![docs.rs](https://docs.rs/machinery-api/badge.svg)](https://docs.rs/machinery/) - Generated API types for The Machinery game engine.
- [![Crates.io](https://img.shields.io/crates/v/machinery-macros.svg?label=machinery-macros)](https://crates.io/crates/machinery-macros) [![docs.rs](https://docs.rs/machinery-macros/badge.svg)](https://docs.rs/machinery-macros/) - Macro utilities for the `machinery` and `machinery-api` crates.
- `machinery-generator` - Bindings generator for The Machinery headers.
- `example` - Short example plugin showcasing the bindings.

## Building Example

The included example depends on [cargo-make](https://github.com/sagiegurari/cargo-make) to copy the
plugin to the plugins directory automatically.

```
cd example
cargo make machinery
```

## Generator

***This generator will soon be re-written when the official API header parser becomes available.***

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
