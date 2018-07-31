#![allow(unknown_lints)]

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
mod load_fns;
mod options;
mod parsing;
mod validation;
mod value;

use std::path::Path;
use value::GenericStruct;

pub use error::{Error, GenerationError, OptionsError};
pub use format::Format;
pub use options::{DynamicLoading, Options};

pub fn generate_config<P: AsRef<Path>>(filepath: P, options: &Options) -> Result<String, Error> {
    let format = Format::from_filename(filepath.as_ref())?;

    generate_config_with_format(format, filepath, options)
}

pub fn generate_config_with_format<P: AsRef<Path>>(
    format: Format,
    filepath: P,
    options: &Options,
) -> Result<String, Error> {
    let path = filepath.as_ref();
    let source = std::fs::read_to_string(path)?;
    let output = generate_config_from_source_with_filepath(format, &source, options, Some(path))?;

    Ok(output)
}

pub fn generate_config_from_source<S: AsRef<str>>(
    format: Format,
    source: S,
    options: &Options,
) -> Result<String, GenerationError> {
    generate_config_from_source_with_filepath(format, source.as_ref(), options, None)
}

fn generate_config_from_source_with_filepath(
    format: Format,
    source: &str,
    options: &Options,
    filepath: Option<&Path>,
) -> Result<String, GenerationError> {
    options.validate()?;

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
        root_struct
    };

    validation::validate_struct(&config)?;

    let mut code = String::new();

    const HEADER: &str = "#![allow(dead_code)]\n\nuse std::borrow::Cow;\n\n";
    code.push_str(HEADER);

    let structs = generation::generate_structs(&config, options);
    code.push_str(&structs);

    let requires_const =
        options.generate_load_fns && options.dynamic_loading != DynamicLoading::Always;

    let struct_name = &options.struct_name;
    let const_name = &options.real_const_name();

    if options.generate_const || requires_const {
        code.push_str(&format!(
            "pub const {}: {} = {};\n",
            const_name,
            struct_name,
            generation::struct_value_string(&config, 0)
        ));
    }

    if options.generate_load_fns {
        let filepath = filepath.ok_or(GenerationError::MissingFilePath);

        let dynamic_impl =
            filepath.map(|path| load_fns::dynamic_load_impl(format, struct_name, path));

        let static_impl = load_fns::static_load_impl(struct_name, const_name);

        let impl_string = match options.dynamic_loading {
            DynamicLoading::Always => dynamic_impl?,
            DynamicLoading::Never => static_impl,
            DynamicLoading::DebugOnly => format!(
                "
#[cfg(debug_assertions)]
{}

#[cfg(not(debug_assertions))]
{}
",
                dynamic_impl?, static_impl,
            ),
        };

        code.push_str(&impl_string);
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
