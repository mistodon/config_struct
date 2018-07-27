use std::io::Error as IOError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Generation error: {}", _0)]
    Generation(#[cause] GenerationError),

    #[fail(display = "IO error: {}", _0)]
    IO(#[cause] IOError),
}

#[derive(Debug, Fail)]
pub enum GenerationError {
    #[fail(
        display = "Unknown input format: `{}`. (Maybe you need to enable the right feature?)", _0
    )]
    UnknownInputFormat(String),

    #[fail(display = "Invalid field name: `{}`.", _0)]
    InvalidFieldName(String),

    #[fail(
        display = "Array under key `{}` has elements of different types. Arrays must be homogenous.",
        _0
    )]
    HeterogenousArray(String),

    #[fail(display = "Deserialization failed: {}", _0)]
    DeserializationFailed(String),

    #[fail(display = "Invalid options error: {}", _0)]
    Options(#[cause] OptionsError),
}

#[derive(Debug, Fail)]
pub enum OptionsError {
    #[fail(display = "Invalid name for a struct: `{}`.", _0)]
    InvalidStructName(String),

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
        GenerationError::Options(error)
    }
}
