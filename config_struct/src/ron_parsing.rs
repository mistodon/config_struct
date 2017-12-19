//! Not all of the RON syntax is currently supported:
//!
//! 1.  Maps are not supported, for example: `{ "a": 1 }`, because `ron` cannot parse them as
//!     structs.
//! 2.  Named structs are not supported, for example: `Person(age: 20)`, because the struct name
//!     is not available at build time, and so cannot match the name in the config file.
//! 3.  Tuples are not supported, for example: `(1, 2, 3)`. It was attempted and did not work for
//!     some reason.


use std::path::Path;

use failure::Error;
use ron::de;
use ron::value::Value;

use { RawValue, RawStructValue };


pub fn parse_config<S: AsRef<str>>(config_source: S) -> Result<RawStructValue, Error>
{
    use std::collections::BTreeMap;

    let ron_object: BTreeMap<String, Value> = {
        let ron_object: Value = de::from_str(config_source.as_ref())?;

        if let Value::Map(mapping) = ron_object
        {
            mapping.into_iter().map(
                |(key, value)|
                {
                    let key = if let Value::String(key) = key { key } else { unimplemented!("Error please!") };
                    (key, value)
                }).collect()
        }
        else
        {
            unimplemented!("Error here please")
        }
    };

    let raw_config: RawStructValue = {
        let struct_name = "Config".to_owned();
        let fields = ron_object.into_iter().map(|(key, value)|
        {
            let value = ron_to_raw_value("_Config", &key, value);
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


fn ron_to_raw_value(super_struct: &str, super_key: &str, value: Value) -> RawValue
{
    match value
    {
        Value::Unit => RawValue::Unit,
        Value::Bool(value) => RawValue::Bool(value),
        Value::Char(value) => RawValue::Char(value),
        Value::Number(value) => {
            let float = value.get();

            if float.trunc() == float { RawValue::I64(float as i64) } else { RawValue::F64(float) }
        },
        Value::String(value) => RawValue::String(value),
        Value::Option(option) => {
            RawValue::Option(option.map(
                |value| Box::new(ron_to_raw_value(super_struct, super_key, *value))))
        },
        Value::Seq(values) => {
            RawValue::Array(values.into_iter()
                .map(|value| ron_to_raw_value(super_struct, super_key, value))
                .collect())
        },
        Value::Map(values) => {
            let sub_struct_name = format!("{}__{}", super_struct, super_key);
            let values = values.into_iter()
                .map(
                    |(key, value)|
                    {
                        let key = {
                            if let Value::String(key) = key
                            {
                                key
                            }
                            else
                            {
                                unimplemented!("We should handle an error here");
                            }
                        };
                        let value = ron_to_raw_value(&sub_struct_name, &key, value);
                        (key, value)
                    })
                .collect();
            RawValue::Struct(RawStructValue { struct_name: sub_struct_name, fields: values })
        }
    }
}

