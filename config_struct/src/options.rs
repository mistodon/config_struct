/// The set of options for generating a config struct.
#[derive(Debug, Clone)]
pub struct Options
{
    /// The name of the resulting struct.
    ///
    /// Defaults to `"Config"`.
    pub struct_name: String,

    /// The name of the const instance of the resulting struct.
    ///
    /// Defaults to `None`, which corresponds to `struct_name` in uppercase.
    pub const_name: Option<String>,

    /// A list of the traits to derive on the resulting struct.
    ///
    /// Defaults to `["Debug", "Clone", "Serialize", "Deserialize"]`.
    pub derived_traits: Vec<String>,

    /// Whether to write the config module even if it is unchanged.
    ///
    /// Defaults to `false`.
    pub always_write: bool,
}

impl Default for Options
{
    fn default() -> Self
    {
        Options
        {
            struct_name: "Config".to_owned(),
            const_name: None,
            derived_traits: vec![
                "Debug".to_owned(),
                "Clone".to_owned(),
                "Serialize".to_owned(),
                "Deserialize".to_owned(),
            ],
            always_write: false,
        }
    }
}

