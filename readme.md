# machinery-rs

Rust bindings for [The Machinery](https://ourmachinery.com/).


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
