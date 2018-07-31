use std::path::Path;

use error::*;

/// Represents an input markup format for a config file.
///
/// The variants that exist correspond to the features that have been enabled.
/// For example, if the `json-parsing` feature is not enabled, then the
/// `Format::Json` variant will not exist.
#[derive(Debug, Clone, Copy)]
pub enum Format {
    #[cfg(feature = "json-parsing")]
    Json,
    #[cfg(feature = "ron-parsing")]
    Ron,
    #[cfg(feature = "toml-parsing")]
    Toml,
    #[cfg(feature = "yaml-parsing")]
    Yaml,
}

impl Format {
    pub fn from_filename(filename: &Path) -> Result<Self, GenerationError> {
        match filename.extension() {
            Some(ext) => match ext.to_string_lossy().as_ref() {
                #[cfg(feature = "json-parsing")]
                "json" => Ok(Format::Json),

                #[cfg(feature = "ron-parsing")]
                "ron" => Ok(Format::Ron),

                #[cfg(feature = "toml-parsing")]
                "toml" => Ok(Format::Toml),

                #[cfg(feature = "yaml-parsing")]
                "yaml" | "yml" => Ok(Format::Yaml),

                other => Err(GenerationError::UnknownInputFormat(other.into())),
            },
            None => Err(GenerationError::UnknownInputFormat("<none>".into())),
        }
    }
}
