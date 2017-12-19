use std::path::Path;

use failure::Error;
use serde_yaml::{ self, Value };

use { RawValue, RawStructValue };


pub fn parse_config<S: AsRef<str>>(config_source: S) -> Result<RawStructValue, Error>
{
    use std::collections::BTreeMap;

    let yaml_object: BTreeMap<String, Value> = serde_yaml::from_str(config_source.as_ref())?;

    let raw_config: RawStructValue = {
        let struct_name = "Config".to_owned();
        let fields = yaml_object.into_iter().map(|(key, value)|
        {
            let value = yaml_to_raw_value("_Config", &key, value);
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

