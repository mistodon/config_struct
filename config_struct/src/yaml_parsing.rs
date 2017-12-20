//! Parsing utilities for YAML config files. (Requires the `yaml-parsing` feature.)

use std::path::Path;

use failure::Error;
use serde_yaml::{ self, Value };

use { RawValue, RawStructValue };


/// Parse a RawStructValue from some YAML.
///
/// This can then be used to generate a config struct using `create_config_module` or
/// `write_config_module`.
pub fn parse_config<S: AsRef<str>>(config_source: S) -> Result<RawStructValue, Error>
{
    use parsing::{ self, ParsedConfig };

    let yaml_object: ParsedConfig<Value> = serde_yaml::from_str(config_source.as_ref())?;

    let raw_config = parsing::parsed_to_raw_config(yaml_object, yaml_to_raw_value);

    Ok(raw_config)
}


/// Parse a RawStructValue from a YAML file.
///
/// This can then be used to generate a config struct using `create_config_module` or
/// `write_config_module`.
pub fn parse_config_from_file<P: AsRef<Path>>(config_path: P) -> Result<RawStructValue, Error>
{
    use parsing;

    let config_source = parsing::slurp_file(config_path.as_ref())?;

    parse_config(&config_source)
}


fn yaml_to_raw_value(super_struct: &str, super_key: &str, value: Value) -> RawValue
{
    match value
    {
        Value::Null => RawValue::Option(None),
        Value::Bool(value) => RawValue::Bool(value),
        Value::Number(value) => match (value.as_i64(), value.as_u64(), value.as_f64())
        {
            (Some(x), _, _) => RawValue::I64(x),
            (None, Some(x), _) => RawValue::U64(x),
            (None, None, Some(x)) => RawValue::F64(x),
            _ => unimplemented!("Should handle error here")
        },
        Value::String(value) => RawValue::String(value),
        Value::Sequence(values) => {
            RawValue::Array(values.into_iter()
                .map(|value| yaml_to_raw_value(super_struct, super_key, value))
                .collect())
        },
        Value::Mapping(values) => {
            let sub_struct_name = format!("{}__{}", super_struct, super_key);
            let values = values.into_iter()
                .map(
                    |(key, value)|
                    {
                        let key = key.as_str().unwrap().to_owned();
                        let value = yaml_to_raw_value(&sub_struct_name, &key, value);
                        (key, value)
                    })
                .collect();
            RawValue::Struct(RawStructValue { struct_name: sub_struct_name, fields: values })
        }
    }
}

