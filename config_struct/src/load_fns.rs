use format::Format;
use std::path::Path;

pub fn dynamic_load_impl(format: Format, struct_name: &str, filepath: &Path) -> String {
    let load_expression = match format {
        #[cfg(feature = "json-parsing")]
        Format::Json => "::serde_json::from_str(&file_contents)",

        #[cfg(feature = "ron-parsing")]
        Format::Ron => "::ron::de::from_str(&file_contents)",

        #[cfg(feature = "toml-parsing")]
        Format::Toml => "::toml::from_str(&file_contents)",

        #[cfg(feature = "yaml-parsing")]
        Format::Yaml => "::serde_yaml::from_str(&file_contents)",
    };

    format!(
r#"impl {struct_name} {{
    pub fn load() -> Cow<'static, Self> {{
        let filepath = concat!(env!("CARGO_MANIFEST_DIR"), "/{filepath}");
        Self::load_from(filepath.as_ref()).expect("Failed to load {struct_name}.")
    }}

    pub fn load_from(filepath: &::std::path::Path) -> Result<Cow<'static, Self>, Box<::std::error::Error>> {{
        let file_contents = ::std::fs::read_to_string(filepath)?;
        let result: Self = {load_expression}?;
        Ok(Cow::Owned(result))
    }}
}}"#, struct_name=struct_name, filepath=filepath.display(), load_expression=load_expression)
}

pub fn static_load_impl(struct_name: &str, const_name: &str) -> String {
    format!(
r#"impl {struct_name} {{
    #[inline(always)]
    pub fn load() -> Cow<'static, Self> {{
        Cow::Borrowed(&{const_name})
    }}

    #[inline(always)]
    pub fn load_from(_: &::std::path::Path) -> Result<Cow<'static, Self>, Box<::std::error::Error>> {{
        Ok(Cow::Borrowed(&{const_name}))
    }}
}}"#, struct_name=struct_name, const_name=const_name)
}
