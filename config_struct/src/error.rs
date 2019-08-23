use std::io::Error as IOError;

use failure::Fail;

/// An error type for errors while generating config struct modules.
///
/// Errors can either occur during IO (when reading or creating files) or during
/// the generation itself.
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Generation error: {}", _0)]
    Generation(#[cause] GenerationError),

    #[fail(display = "IO error: {}", _0)]
    IO(#[cause] IOError),
}

/// An error occurring during code generation.
#[derive(Debug, Fail)]
pub enum GenerationError {
    /// Occurs when the config format can't be determined from the
    /// filename extension of the input file.
    #[fail(
        display = "Unknown input format: `{}`. (Maybe you need to enable the right feature?)",
        _0
    )]
    UnknownInputFormat(String),

    /// Occurs when encountering a field in the config which is not a
    /// valid name for a struct field.
    #[fail(display = "Invalid field name: `{}`.", _0)]
    InvalidFieldName(String),

    /// Occurs when an array in the config file contains multiple different types
    /// of data, which cannot be represented in a Rust struct.
    #[fail(
        display = "Array under key `{}` has elements of different types. Arrays must be homogenous.",
        _0
    )]
    HeterogenousArray(String),

    /// Occurs when generating from source and not a file, if attempting to also
    /// generate dynamic loading functions.
    ///
    /// Because no input filepath was given, it's impossible to generate a function
    /// which loads from that file.
    #[fail(
        display = "Cannot generate dynamic loading functions without a filename.
(Generate struct from a file, set generate_load_fns: false, or set dynamic_loading: DynamicLoading::Never to fix.)"
    )]
    MissingFilePath,

    /// Occurs when the config file could not be correctly parsed.
    #[fail(display = "Deserialization failed: {}", _0)]
    DeserializationFailed(String),

    /// Occurs when invalid options were provided.
    #[fail(display = "Invalid options error: {}", _0)]
    StructOptions(#[cause] OptionsError),
}

/// An error type for when a [`StructOptions`](struct.StructOptions.html) value
/// failed validation.
#[derive(Debug, Fail)]
pub enum OptionsError {
    /// Occurs when the provided `struct_name` is not a valid Rust identifier.
    #[fail(display = "Invalid name for a struct: `{}`.", _0)]
    InvalidStructName(String),

    /// Occurs when the provided `const_name` is not a valid Rust identifier.
    #[fail(display = "Invalid name for a const: `{}`.", _0)]
    InvalidConstName(String),
}

impl From<GenerationError> for Error {
    fn from(error: GenerationError) -> Self {
        Error::Generation(error)
    }
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Self {
        Error::IO(error)
    }
}

impl From<OptionsError> for GenerationError {
    fn from(error: OptionsError) -> Self {
        GenerationError::StructOptions(error)
    }
}
