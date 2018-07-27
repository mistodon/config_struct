use std::path::Path;

use error::*;

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
            }
            None => Err(GenerationError::UnknownInputFormat("<none>".into())),
        }
    }
}
