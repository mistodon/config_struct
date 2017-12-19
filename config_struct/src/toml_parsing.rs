use std::path::Path;

use failure::Error;
use toml::{ self, Value };

use { RawValue, RawStructValue };


pub fn parse_config<S: AsRef<str>>(config_source: S) -> Result<RawStructValue, Error>
{
    use std::collections::BTreeMap;

    let toml_object: BTreeMap<String, Value> = toml::from_str(config_source.as_ref())?;

    let raw_config: RawStructValue = {
        let struct_name = "Config".to_owned();
        let fields = toml_object.into_iter().map(|(key, value)|
        {
            let value = toml_to_raw_value("_Config", &key, value);
            (key, value)
        }).collect();
        RawStructValue { struct_name, fields }
    };

    Ok(raw_config)
}

pub fn parse_config_from_file<P: AsRef<Path>>(config_path: P) -> Result<RawStructValue, Error>
{
    let config_source = {
        use std::fs::File;
        use std::io::Read;

        let mut buffer = String::new();
        let file = &mut File::open(&config_path)?;
        file.read_to_string(&mut buffer)?;
        buffer
    };

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

