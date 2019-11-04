//! This crate is a library for generating structs based on a config
//! file at build time. It is intended for use in a `build.rs` file
//! so should be included in your `[build-dependencies]`.
//!
//! ```toml
//! [build-dependencies.config_struct]
//! version = "~0.3.0"
//! features = ["toml-parsing"]
//! ```
//!
//! By default, `config_struct` is markup-language-agnostic, so
//! include the relevant feature for whatever language your config
//! file is written in. Choices are:
//!
//! 1.  `json-parsing`
//! 2.  `ron-parsing`
//! 3.  `toml-parsing`
//! 4.  `yaml-parsing`
//!
//! Only `toml-parsing` is included by default, so be sure to specify
//! the features you need in your `Cargo.toml` file.
//!
//! # Examples
//!
//! ```rust,no_run
//! // build.rs
//! use config_struct::{Error, StructOptions};
//!
//! fn main() -> Result<(), Error> {
//!     config_struct::create_struct(
//!         "config.toml",
//!         "src/config.rs",
//!         &StructOptions::default())
//! }
//! ```
//!
//! The above build script will take the following `config.toml` file and generate
//! a `config.rs` like the following:
//!
//! ```toml
//! # config.toml
//! name = "Application"
//! version = 5
//! features = [
//!     "one",
//!     "two",
//!     "three"
//! ]
//! ```
//!
//! ```rust,no_run
//! // config.rs
//! // ...
//! use std::borrow::Cow;
//!
//! #[derive(Debug, Clone)]
//! #[allow(non_camel_case_types)]
//! pub struct Config {
//!     pub features: Cow<'static, [Cow<'static, str>]>,
//!     pub name: Cow<'static, str>,
//!     pub version: i64,
//! }
//!
//! pub const CONFIG: Config = Config {
//!     features: Cow::Borrowed(&[Cow::Borrowed("one"), Cow::Borrowed("two"), Cow::Borrowed("three")]),
//!     name: Cow::Borrowed("Application"),
//!     version: 5,
//! };
//! ```
//!
//! Strings and arrays are represented by `Cow` types, which allows
//! the entire Config struct to be either heap allocated at runtime,
//! or a compile time constant, as shown above.

#[cfg(feature = "json-parsing")]
mod json_parsing;

#[cfg(feature = "ron-parsing")]
mod ron_parsing;

#[cfg(feature = "toml-parsing")]
mod toml_parsing;

#[cfg(feature = "yaml-parsing")]
mod yaml_parsing;

mod enums;
mod structs;

mod error;
mod files;
mod format;
mod generation;
mod load_fns;
mod options;
mod parsing;
mod validation;
mod value;

#[cfg(not(any(
    feature = "json-parsing",
    feature = "ron-parsing",
    feature = "toml-parsing",
    feature = "yaml-parsing"
)))]
compile_error!("The config_struct crate requires at least one parsing feature to be enabled:\n {json-parsing, ron-parsing, toml-parsing, yaml-parsing}");

pub use crate::{
    enums::*,
    error::{Error, GenerationError, OptionsError},
    format::Format,
    options::{DynamicLoading, FloatSize, IntSize, SerdeSupport, StructOptions, EnumOptions},
    structs::*,
};
