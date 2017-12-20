//! Parsing utilities for TOML config files. (Requires the `toml-parsing` feature.)

use std::path::Path;

use failure::Error;
use toml::{ self, Value };

use { RawValue, RawStructValue };


/// Parse a RawStructValue from some TOML.
///
/// This can then be used to generate a config struct using `create_config_module` or
/// `write_config_module`.
pub fn parse_config<S: AsRef<str>>(config_source: S) -> Result<RawStructValue, Error>
{
    use parsing::{ self, ParsedConfig };

    let toml_object: ParsedConfig<Value> = toml::from_str(config_source.as_ref())?;

    let raw_config = parsing::parsed_to_raw_config(toml_object, toml_to_raw_value);

    Ok(raw_config)
}


/// Parse a RawStructValue from a TOML file.
///
/// This can then be used to generate a config struct using `create_config_module` or
/// `write_config_module`.
pub fn parse_config_from_file<P: AsRef<Path>>(config_path: P) -> Result<RawStructValue, Error>
{
    use parsing;

    let config_source = parsing::slurp_file(config_path.as_ref())?;

    parse_config(&config_source)
}


fn toml_to_raw_value(super_struct: &str, super_key: &str, value: Value) -> RawValue
{
    match value
    {
        Value::Boolean(value) => RawValue::Bool(value),
        Value::Integer(value) => RawValue::I64(value),
        Value::Float(value) => RawValue::F64(value),
        Value::String(value) => RawValue::String(value),
        Value::Datetime(value) => RawValue::String(value.to_string()),
        Value::Array(values) => {
            RawValue::Array(values.into_iter()
                .map(|value| toml_to_raw_value(super_struct, super_key, value))
                .collect())
        },
        Value::Table(values) => {
            let sub_struct_name = format!("{}__{}", super_struct, super_key);
            let values = values.into_iter()
                .map(
                    |(key, value)|
                    {
                        let value = toml_to_raw_value(&sub_struct_name, &key, value);
                        (key, value)
                    })
                .collect();
            RawValue::Struct(RawStructValue { struct_name: sub_struct_name, fields: values })
        }
    }
}

