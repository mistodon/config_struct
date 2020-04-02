use std::path::Path;

use crate::{
    error::{Error, GenerationError},
    files,
    format::Format,
    generation, load_fns,
    options::{DynamicLoading, StructOptions},
    validation,
    value::GenericStruct,
};

/// Generate Rust source code defining structs based on a config file.
///
/// The format of the config file will be auto-detected from its extension.
///
/// # Examples
/// ```rust,no_run
/// # fn main() -> Result<(), config_struct::Error> {
/// let code = config_struct::generate_struct("config.toml", &Default::default())?;
/// assert!(code.contains("pub struct Config"));
/// # Ok(())
/// # }
/// ```
pub fn generate_struct<P: AsRef<Path>>(
    filepath: P,
    options: &StructOptions,
) -> Result<String, Error> {
    let path = filepath.as_ref();
    let source = std::fs::read_to_string(path)?;
    let output = generate_struct_from_source_with_filepath(&source, options, Some(path))?;

    Ok(output)
}

/// Generate Rust source code defining structs from a config string
/// in a format specified in the provided options.
///
/// # Examples
/// ```rust
/// # fn main() -> Result<(), config_struct::Error> {
/// use config_struct::{StructOptions, Format};
///
/// let code = config_struct::generate_struct_from_source(
///     "number = 100  # This is valid TOML.",
///     &StructOptions {
///         format: Some(Format::Toml),
///         ..Default::default()
///     })?;
///
/// assert!(code.contains("pub struct Config"));
/// assert!(code.contains("pub number: i64"));
/// assert!(code.contains("number: 100"));
/// # Ok(())
/// # }
/// ```
pub fn generate_struct_from_source<S: AsRef<str>>(
    source: S,
    options: &StructOptions,
) -> Result<String, GenerationError> {
    generate_struct_from_source_with_filepath(source.as_ref(), options, None)
}

fn generate_struct_from_source_with_filepath(
    source: &str,
    options: &StructOptions,
    filepath: Option<&Path>,
) -> Result<String, GenerationError> {
    options.validate()?;

    let format = match options.format {
        Some(format) => format,
        None => match filepath {
            Some(path) => Format::from_filename(path)?,
            None => return Err(GenerationError::UnknownInputFormat("<none>".into())),
        }
    };

    let config = {
        let mut root_struct: GenericStruct = match format {
            #[cfg(feature = "json-parsing")]
            Format::Json => crate::json_parsing::parse_json(source, options)?,

            #[cfg(feature = "ron-parsing")]
            Format::Ron => crate::ron_parsing::parse_ron(source, options)?,

            #[cfg(feature = "toml-parsing")]
            Format::Toml => crate::toml_parsing::parse_toml(source, options)?,

            #[cfg(feature = "yaml-parsing")]
            Format::Yaml => crate::yaml_parsing::parse_yaml(source, options)?,
        };
        root_struct.struct_name = options.struct_name.clone();
        root_struct
    };

    validation::validate_struct(&config)?;

    let mut code = String::new();

    const HEADER: &str = "#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]

use std::borrow::Cow;\n\n";
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
            generation::struct_value_string(&config, 0, options.max_array_size)
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

/// Generate a Rust module containing struct definitions based on a
/// given config file.
///
/// The format of the config is auto-detected from its filename
/// extension.
///
/// # Examples
///
/// ```rust,no_run
/// # fn main() -> Result<(), config_struct::Error> {
/// use config_struct::StructOptions;
///
/// config_struct::create_struct("config.toml", "src/config.rs", &StructOptions::default())?;
/// # Ok(())
/// # }
/// ```
pub fn create_struct<SrcPath: AsRef<Path>, DstPath: AsRef<Path>>(
    filepath: SrcPath,
    destination: DstPath,
    options: &StructOptions,
) -> Result<(), Error> {
    let output = generate_struct(filepath, options)?;
    files::ensure_destination(destination.as_ref(), options.create_dirs)?;
    files::write_destination(destination.as_ref(), output, options.write_only_if_changed)?;

    Ok(())
}

/// Generate a Rust module containing struct definitions from a
/// config string in a format specified by the provided options.
///
/// # Examples
///
/// ```rust,no_run
/// # fn main() -> Result<(), config_struct::Error> {
/// use config_struct::{Format, StructOptions};
///
/// config_struct::create_struct_from_source(
///     "number = 100  # This is valid TOML.",
///     "src/config.rs",
///     &StructOptions {
///         format: Some(Format::Toml),
///         ..Default::default()
///     })?;
/// # Ok(())
/// # }
/// ```
pub fn create_struct_from_source<S: AsRef<str>, P: AsRef<Path>>(
    source: S,
    destination: P,
    options: &StructOptions,
) -> Result<(), Error> {
    let output = generate_struct_from_source(source, options)?;
    files::ensure_destination(destination.as_ref(), options.create_dirs)?;
    files::write_destination(destination.as_ref(), output, options.write_only_if_changed)?;

    Ok(())
}
