#[cfg(feature = "json-parsing")]
extern crate serde_json;

#[cfg(feature = "ron-parsing")]
extern crate ron;

#[cfg(feature = "toml-parsing")]
extern crate toml;

#[cfg(feature = "yaml-parsing")]
extern crate serde_yaml;

#[macro_use]
extern crate failure;

#[cfg(feature = "json-parsing")]
mod json_parsing;

#[cfg(feature = "ron-parsing")]
mod ron_parsing;

#[cfg(feature = "toml-parsing")]
mod toml_parsing;

#[cfg(feature = "yaml-parsing")]
mod yaml_parsing;

mod error;
mod format;
mod generation;
mod options;
mod parsing;
mod validation;
mod value;

use std::path::Path;
use value::{GenericStruct, ParsedConfig};

pub use error::{Error, GenerationError, OptionsError};
pub use format::Format;
pub use options::Options;

pub fn generate_config<P: AsRef<Path>>(filepath: P, options: &Options) -> Result<String, Error> {
    let format = Format::from_filename(filepath.as_ref())?;

    generate_config_with_format(format, filepath, options)
}

pub fn generate_config_with_format<P: AsRef<Path>>(
    format: Format,
    filepath: P,
    options: &Options,
) -> Result<String, Error> {
    let source = std::fs::read_to_string(filepath.as_ref())?;
    let output = generate_config_from_source(format, source, options)?;

    Ok(output)
}

pub fn generate_config_from_source<S: AsRef<str>>(
    format: Format,
    source: S,
    options: &Options,
) -> Result<String, GenerationError> {
    options.validate()?;

    let source = source.as_ref();
    let config = {
        let mut root_struct: GenericStruct = match format {
            #[cfg(feature = "json-parsing")]
            Format::Json => json_parsing::parse_json(source, options)?,

            #[cfg(feature = "ron-parsing")]
            Format::Ron => ron_parsing::parse_ron(source, options)?,

            #[cfg(feature = "toml-parsing")]
            Format::Toml => toml_parsing::parse_toml(source, options)?,

            #[cfg(feature = "yaml-parsing")]
            Format::Yaml => yaml_parsing::parse_yaml(source, options)?,
        };
        root_struct.struct_name = options.struct_name.clone();

        ParsedConfig {
            filename: None, // TODO: Fix this
            format,
            root_struct,
        }
    };

    validation::validate_struct(&config.root_struct)?;

    let mut code = String::new();

    const IMPORTS: &str = "use std::borrow::Cow;\n\n";
    code.push_str(IMPORTS);

    let structs = generation::generate_structs(&config.root_struct, options);
    code.push_str(&structs);

    if options.generate_const {
        code.push_str(&format!(
            "pub const {}: {} = {};\n",
            options.real_const_name(),
            options.struct_name,
            generation::struct_value_string(&config.root_struct, 0)
        ));
    }

    Ok(code)
}

pub fn create_config<SrcPath: AsRef<Path>, DstPath: AsRef<Path>>(
    filepath: SrcPath,
    destination: DstPath,
    options: &Options,
) -> Result<(), Error> {
    let output = generate_config(filepath, options)?;
    std::fs::write(destination, output)?;
    Ok(())
}

pub fn create_config_with_format<SrcPath: AsRef<Path>, DstPath: AsRef<Path>>(
    format: Format,
    filepath: SrcPath,
    destination: DstPath,
    options: &Options,
) -> Result<(), Error> {
    let output = generate_config_with_format(format, filepath, options)?;
    std::fs::write(destination, output)?;
    Ok(())
}

pub fn create_config_from_source<S: AsRef<str>, P: AsRef<Path>>(
    format: Format,
    source: S,
    destination: P,
    options: &Options,
) -> Result<(), Error> {
    let output = generate_config_from_source(format, source, options)?;
    std::fs::write(destination, output)?;
    Ok(())
}
