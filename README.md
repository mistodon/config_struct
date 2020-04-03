config_struct
===

This is a library for converting config files into matching source files at build time.

[![Build Status](https://travis-ci.org/mistodon/config_struct.svg?branch=master)](https://travis-ci.org/mistodon/config_struct)
[![Crates.io](https://img.shields.io/crates/v/config_struct.svg)](https://crates.io/crates/config_struct)
[![Docs.rs](https://docs.rs/config_struct/badge.svg)](https://docs.rs/config_struct/0.4.0/config_struct/)

## Usage

This library is intended to be used in a `build.rs` file, so it needs to be added to `[build-dependencies]`.

```toml
[build-dependencies.config_struct]
version = "~0.4.0"
features = ["toml-parsing"]
```

By default, `config_struct` is markup-language-agnostic, so include the relevant feature for whatever language your config file is written in. Choices are:

1.  `json-parsing`
2.  `ron-parsing`
3.  `toml-parsing`
4.  `yaml-parsing`

### Build-time

Now in your `build.rs` file, add code like the following:

```rust
use config_struct::{Error, StructOptions};

fn main() -> Result<(), Error> {
    config_struct::create_config(
        "config.toml",
        "src/config.rs",
        &StructOptions::default())
}
```

This will take the following `config.toml` file:

```toml
name = "Config name"
```

... and generate a `config.rs` file like the following:

```rust
// ...
use std::borrow::Cow;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct Config {
    pub name: Cow<'static, str>,
}

pub const CONFIG: Config = Config {
    name: Cow::Borrowed("Config name"),
};
```

Strings and arrays are represented by `Cow` types, which allows the entire Config struct to be either heap allocated at runtime, or a compile time constant, as shown above.

#### Support for `serde`

Unless you are specifically avoiding the `serde` family of crates at runtime, it's recommended to use the following options:

```rust
StructOptions {
    serde_support: SerdeSupport::Yes,
    generate_load_fns: true,
    .. my_other_options
}
```

This will derive the `Serialize` and `Deserialize` traits for your struct, as well as providing a handy `load()` method to read and parse the file at runtime.

If these are the only options you want to set beyond the defaults, you can use `StructOptions::serde_default()` as a shorthand.

### Runtime

There are a few different ways to access the config at runtime.

1.  Call the generated load function, e.g. `let config = Config::load();`
    - Note that this requires the `generate_load_fns` option described above.
2.  Access the `CONFIG` const directly, e.g. `let x = CONFIG.name;`
3.  Deserialize the config file manually, e.g. `let config: Config = toml::from_str(file_contents)?`
    - Note that this either requires the `serde_support` option above, or requires you to manually add `serde::Serialize` and `serde::Deserialize` to the `derived_traits` option.

The first method is recommended, as it will return the const value in release mode, but load from the filesystem in debug mode. This gives you flexibility during development and immutability in release.

### Enums

Enum generation works in a similar way to structs, but for the keys of a map.

```rust
// build.rs
use config_struct::{Error, EnumOptions};

fn main() -> Result<(), Error> {
    config_struct::create_enum(
        "items.yaml",
        "src/items.rs",
        &EnumOptions::default())
}
```

The above build script will take the following `items.yaml` file and generate
a (not-formatted) `items.rs` like the following:

```yaml
# items.yaml
ItemOne:
    - data
ItemTwo:
    - more
    - data
```

```rust
// items.rs
// ...
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    ItemOne,
    ItemTwo,
}
impl Key {
    pub const ALL: &'static [Key] = &[Key::ItemOne, Key::ItemTwo];
}
impl Default for Key {
    fn default() -> Self {
        Self::ItemOne
    }
}
impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::str::FromStr for Key {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const STRINGS: &'static [&'static str] = &["ItemOne", "ItemTwo"];
        for (index, &key) in STRINGS.iter().enumerate() {
            if key == s {
                return Ok(Key::ALL[index]);
            }
        }
        Err(())
    }
}
```

As you can see, this provides more functionality out-of-the-box - most of
which could be disabled in the `EnumOptions`. The intended purpose of
this is to have a small efficient type to use as a key into the data stored
in the initial config file.
