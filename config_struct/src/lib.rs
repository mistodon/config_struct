//! This crate is a library for generating structs based on a config file at build time.
//! It is intended for use in a `build.rs` file so should be included in your
//! `[build-dependencies]`.
//!
//! The core library is agnostic to the markup language of the config, but there are a few
//! features included to make it easier to use with a few markup languages:
//!
//! 1.  `json-parsing`
//! 2.  `ron-parsing`
//! 3.  `toml-parsing`
//! 4.  `yaml-parsing`
//!
//! None of these are included by default, so be sure to include one in your `Cargo.toml`.
//!
//! # Examples
//!
//! ```rust,ignore
//! // build.rs
//! extern crate config_struct;
//!
//! use config_struct::toml_parsing;
//!
//! fn main() {
//!     let toml_config = toml_parsing::parse_config_from_file("config.toml").unwrap();
//!
//!     config_struct::write_config_module(
//!         "src/config.rs",
//!         &toml_config,
//!         &Default::default()).unwrap();
//! }
//! ```
//!
//! The above build script will take the following `config.toml` file and generate `config.rs`:
//!
//! ```toml
//! // config.toml
//! name = "Application"
//! version = 5
//! features = [
//!     "one",
//!     "two",
//!     "three"
//! ]
//! ```
//!
//! ```rust,ignore
//! // config.rs
//! use std::borrow::Cow;
//!
//! #[derive(Debug, Clone, Serialize, Deserialize)]
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
//! Strings and arrays are represented by `Cow` types, which allows the entire Config struct to
//! be either heap allocated at runtime, or a compile time constant, as shown above.


#[cfg(feature = "json-parsing")]
extern crate serde_json;

#[cfg(feature = "ron-parsing")]
extern crate ron;

#[cfg(feature = "toml-parsing")]
extern crate toml;

#[cfg(feature = "yaml-parsing")]
extern crate serde_yaml;


#[cfg(feature = "json-parsing")]
pub mod json_parsing;

#[cfg(feature = "ron-parsing")]
pub mod ron_parsing;

#[cfg(feature = "toml-parsing")]
pub mod toml_parsing;

#[cfg(feature = "yaml-parsing")]
pub mod yaml_parsing;


#[macro_use]
extern crate failure;


#[cfg(any(feature = "json-parsing", feature = "ron-parsing", feature = "toml-parsing", feature = "yaml-parsing"))]
mod parsing;

mod generation;
mod options;
mod validation;
mod value;


use std::path::Path;

use failure::Error;

pub use options::{ Options };
pub use validation::StructGenerationError;
pub use value::{ RawValue, RawStructValue };


/// Generate Rust code for a RawStructValue.
///
/// This will recursively generate code for all nested structs within the given value.
///
/// The easiest way to get a RawStructValue is to use the `parse_config` or
/// `parse_config_from_file` function from one of the parsing modules.
pub fn create_config_module(
    raw_config: &RawStructValue,
    options: &Options) -> Result<String, StructGenerationError>
{
    validation::validate_options(options)?;

    let raw_config = {
        // TODO: Ugh, this is really ugly
        let mut config = raw_config.clone();
        config.struct_name = options.struct_name.clone();
        config
    };

    validation::validate_struct_value(&raw_config)?;

    let mut code = String::new();

    code.push_str("use std::borrow::Cow;\n\n");

    let structs = generation::generate_structs(&raw_config, options);
    code.push_str(&structs);

    let const_name = options.const_name.clone().unwrap_or(options.struct_name.to_uppercase());

    code.push_str(
        &format!(
            "pub const {}: {} = {};\n",
            const_name,
            options.struct_name,
            generation::struct_value_string(&raw_config, 0)));

    Ok(code)
}


/// Generate Rust code for a RawStructValue and write it to a file.
///
/// This simply writes the result of `create_config_module` to a file.
pub fn write_config_module<P>(
    module_path: P,
    raw_config: &RawStructValue,
    options: &Options) -> Result<(), Error>
where
    P: AsRef<Path>
{
    use std::fs::File;
    use std::io::Write;

    let code = create_config_module(raw_config, options)?;

    let should_write = {
        if options.always_write
        {
            true
        }
        else
        {
            let existing_code = read_entire_file(module_path.as_ref()).unwrap_or(String::default());
            code != existing_code
        }
    };

    if should_write
    {
        let file = &mut File::create(module_path)?;
        file.write_all(code.as_bytes())?;
    }

    Ok(())
}

fn read_entire_file(path: &Path) -> std::io::Result<String>
{
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
