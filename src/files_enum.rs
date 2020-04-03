use std::path::Path;

use case::CaseExt;

use crate::{
    error::{Error, GenerationError},
    files,
    generation,
    options::{EnumOptions},
    validation,
};

pub fn generate_files_enum<P: AsRef<Path>>(
    dirpath: P,
    options: &EnumOptions,
) -> Result<String, Error> {
    let mut keys: Vec<String> = vec![];
    for entry in std::fs::read_dir(dirpath)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            keys.push(
                entry
                    .path()
                    .file_stem()
                    .expect("TODO:")
                    .to_string_lossy()
                    .to_camel()
            );
        }
    }

    // NOTE: Important that they stay sorted by Vec::sort
    keys.sort();

    // TODO: Refactor, Validate other identifiers, DRY-fail - see lib.rs
    {
        for key in &keys {
            if !validation::valid_identifier(key) {
                return Err(GenerationError::InvalidVariantName(key.into()))?;
            }
        }
    }

    let enum_code = generation::generate_enum(&keys, options);

    Ok(enum_code)
}

pub fn create_files_enum<SrcPath: AsRef<Path>, DstPath: AsRef<Path>>(
    dirpath: SrcPath,
    destination: DstPath,
    options: &EnumOptions,
) -> Result<(), Error> {
    let output = generate_files_enum(dirpath, options)?;
    files::ensure_destination(destination.as_ref(), options.create_dirs)?;
    files::write_destination(destination.as_ref(), output, options.write_only_if_changed)?;

    Ok(())
}
