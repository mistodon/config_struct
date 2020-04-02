use std::path::Path;

use crate::{
    error::{Error, GenerationError},
    files,
    format::Format,
    generation,
    options::{EnumOptions},
    validation,
};

/// Generate Rust source code defining an enum based on a map-like config file.
///
/// The enum variants are based on the keys of the map. The format of the
/// config file will be auto-detected from its extension.
///
/// # Examples
/// ```rust,no_run
/// # fn main() -> Result<(), config_struct::Error> {
/// let code = config_struct::generate_enum("map.toml", &Default::default())?;
/// assert!(code.contains("pub struct Key"));
/// # Ok(())
/// # }
/// ```
pub fn generate_enum<P: AsRef<Path>>(
    filepath: P,
    options: &EnumOptions,
) -> Result<String, Error> {
    let path = filepath.as_ref();
    let source = std::fs::read_to_string(path)?;
    let output = generate_enum_from_source_with_filepath(&source, options, Some(path))?;

    Ok(output)
}

/// Generate Rust source code defining an enum from a config string containing
/// a map-like structure. The variants of the enum are based on the keys of
/// the map.
///
/// The format must be specified in the provided options.
///
/// # Examples
/// ```rust
/// # fn main() -> Result<(), config_struct::Error> {
/// use config_struct::{EnumOptions, Format};
///
/// let code = config_struct::generate_enum_from_source(
///     "[KeyOne]\n[KeyTwo]\n",
///     &EnumOptions {
///         format: Some(Format::Toml),
///         ..Default::default()
///     })?;
/// eprintln!("CODE : {}", code);
///
/// assert!(code.contains("pub enum Key"));
/// assert!(code.contains("KeyOne"));
/// assert!(code.contains("KeyTwo"));
/// # Ok(())
/// # }
/// ```
pub fn generate_enum_from_source<S: AsRef<str>>(
    source: S,
    options: &EnumOptions,
) -> Result<String, GenerationError> {
    generate_enum_from_source_with_filepath(source.as_ref(), options, None)
}

fn generate_enum_from_source_with_filepath(
    source: &str,
    options: &EnumOptions,
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

    let keys: Vec<String> = match format {
        #[cfg(feature = "json-parsing")]
        Format::Json => crate::json_parsing::parse_map_keys(source)?,

        #[cfg(feature = "ron-parsing")]
        Format::Ron => crate::ron_parsing::parse_map_keys(source)?,

        #[cfg(feature = "toml-parsing")]
        Format::Toml => crate::toml_parsing::parse_map_keys(source)?,

        #[cfg(feature = "yaml-parsing")]
        Format::Yaml => crate::yaml_parsing::parse_map_keys(source)?,
    };

    // TODO: Refactor
    // TODO: Validate other identifiers
    {
        for key in &keys {
            if !validation::valid_identifier(key) {
                return Err(GenerationError::InvalidVariantName(key.into()))
            }
        }
    }

    let enum_code = generation::generate_enum(&keys, options);

    Ok(enum_code)
}

/// Generate a Rust module containing an enum definition based on a
/// given config file containing a map-like structure. The variants
/// of the enum are based on the keys of the map.
///
/// The format of the config is auto-detected from its filename
/// extension.
///
/// # Examples
///
/// ```rust,no_run
/// # fn main() -> Result<(), config_struct::Error> {
/// use config_struct::EnumOptions;
///
/// config_struct::create_enum("map.toml", "src/keys.rs", &EnumOptions::default())?;
/// # Ok(())
/// # }
/// ```
pub fn create_enum<SrcPath: AsRef<Path>, DstPath: AsRef<Path>>(
    filepath: SrcPath,
    destination: DstPath,
    options: &EnumOptions,
) -> Result<(), Error> {
    let output = generate_enum(filepath, options)?;
    files::ensure_destination(destination.as_ref(), options.create_dirs)?;
    files::write_destination(destination.as_ref(), output, options.write_only_if_changed)?;

    Ok(())
}

/// Generate a Rust module containing an enum definition based on a
/// config string containing a map-like structure. The variants
/// of the enum are based on the keys of the map.
///
/// The format of the config must be provided in the options.
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
pub fn create_enum_from_source<S: AsRef<str>, P: AsRef<Path>>(
    source: S,
    destination: P,
    options: &EnumOptions,
) -> Result<(), Error> {
    let output = generate_enum_from_source(source, options)?;
    files::ensure_destination(destination.as_ref(), options.create_dirs)?;
    files::write_destination(destination.as_ref(), output, options.write_only_if_changed)?;

    Ok(())
}
