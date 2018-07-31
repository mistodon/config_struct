config_struct
===

This is a library for converting config files into matching source files at build time.

[![Build Status](https://travis-ci.org/Mistodon/config_struct.svg?branch=master)](https://travis-ci.org/Mistodon/config_struct)
[![Crates.io](https://img.shields.io/crates/v/config_struct.svg)](https://crates.io/crates/config_struct)

Usage
---

This library is intended to be used in a `build.rs` file, so it needs to be added to `[build-dependencies]`.

```toml
[build-dependencies.config_struct]
version = "~0.1.0"
features = ["toml-parsing"]
```

By default, `config_struct` is markup-language-agnostic, so include the relevant feature for whatever language your config file is written in. Choices are:

1.  `json-parsing`
2.  `ron-parsing`
3.  `toml-parsing`
4.  `yaml-parsing`

Now in your `build.rs` file, add code like the following:

```rust
extern crate config_struct;

fn main() {
    let toml_config = config_struct::toml_parsing::parse_config_from_file("config.toml").unwrap();
    config_struct::write_config_module("src/config.rs", &toml_config, &Default::default()).unwrap();
}
```

This will take the following `config.toml` file:

```toml
name = "Config name"
```

... and generate the following `config.rs` file:

```rust
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Config {
    pub name: Cow<'static, str>,
}

pub const CONFIG: Config = Config {
    name: Cow::Borrowed("Config name"),
};
```

test_config_struct
===

A crate with a build script for testing `config_struct`. Run `cargo test` in that directory.


TODO
===

### For sure
-   Features (functions?) to opt in/out of the generated const
-   Allow generating the const only in release mode
-   Documentation

### Eventually
-   Nicer, more flexible API
-   Fix issues with RON parsing
-   Better error handling
-   Better test coverage

### Like, mayyybe
-   Possibly allow specifying imports, and custom types
-   Try to avoid generating the same struct twice
-   Use fixed-size arrays instead of slices for some (configurable) size threshold
-   If elements of arrays have different keys, take the union and make non-universal/non-matching members Options?
    -   compromise([1.0, 2, 3]) -> [1.0, 2.0, 3.0]
    -   compromise(["Hello", null]) -> [Some("Hello"), None]
    -   compromise([[floats], [ints]] -> [[floats], [floats]])
    -   compromise(["Hello", 1]) -> None // fails

