use std::path::Path;

use crate::{
    error::{Error, GenerationError},
    files,
    format::Format,
    generation,
    options::{EnumOptions},
    validation,
};

pub fn generate_enum<P: AsRef<Path>>(
    filepath: P,
    options: &EnumOptions,
) -> Result<String, Error> {
    let format = Format::from_filename(filepath.as_ref())?;

    generate_enum_with_format(format, filepath, options)
}

pub fn generate_enum_with_format<P: AsRef<Path>>(
    format: Format,
    filepath: P,
    options: &EnumOptions,
) -> Result<String, Error> {
    let path = filepath.as_ref();
    let source = std::fs::read_to_string(path)?;
    let output = generate_enum_from_source(format, &source, options)?;

    Ok(output)
}

pub fn generate_enum_from_source<S: AsRef<str>>(
    format: Format,
    source: S,
    options: &EnumOptions,
) -> Result<String, GenerationError> {
    options.validate()?;

    let source = source.as_ref();
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

pub fn create_enum_with_format<SrcPath: AsRef<Path>, DstPath: AsRef<Path>>(
    format: Format,
    filepath: SrcPath,
    destination: DstPath,
    options: &EnumOptions,
) -> Result<(), Error> {
    let output = generate_enum_with_format(format, filepath, options)?;
    files::ensure_destination(destination.as_ref(), options.create_dirs)?;
    files::write_destination(destination.as_ref(), output, options.write_only_if_changed)?;

    Ok(())
}

pub fn create_enum_from_source<S: AsRef<str>, P: AsRef<Path>>(
    format: Format,
    source: S,
    destination: P,
    options: &EnumOptions,
) -> Result<(), Error> {
    let output = generate_enum_from_source(format, source, options)?;
    files::ensure_destination(destination.as_ref(), options.create_dirs)?;
    files::write_destination(destination.as_ref(), output, options.write_only_if_changed)?;

    Ok(())
}
