//! Parsing utilities for RON config files. (Requires the `ron-parsing` feature.)
//!
//! Not all of the RON syntax is currently supported:
//!
//! 1.  Maps are not supported, for example: `{ "a": 1 }`, because `ron` cannot parse them as
//!     structs.
//! 2.  Named structs are not supported, for example: `Person(age: 20)`, because the struct name
//!     is not available at build time, and so cannot match the name in the config file.
//! 3.  Tuples are not supported, for example: `(1, 2, 3)`. It was attempted and did not work for
//!     some reason.
use ron::{self, value::Value};

use error::GenerationError;
use options::Options;
use parsing;
use value::{GenericStruct, GenericValue};

pub fn parse_ron(ron: &str, options: &Options) -> Result<GenericStruct, GenerationError> {
    use parsing::ParsedFields;

    let ron_struct = {
        let ron_object: Value = ron::de::from_str(ron)
            .map_err(|err| GenerationError::DeserializationFailed(err.to_string()))?;

        if let Value::Map(mapping) = ron_object {
            mapping
                .into_iter()
                .map(|(key, value)| {
                    let key = {
                        if let Value::String(key) = key {
                            key
                        } else {
                            let m = "Top-level keys in RON map must be strings.".to_owned();
                            return Err(GenerationError::DeserializationFailed(m));
                        }
                    };
                    Ok((key, value))
                })
                .collect::<Result<ParsedFields<Value>, GenerationError>>()?
        } else {
            let m = "Root RON object must be a struct or map.".to_owned();
            return Err(GenerationError::DeserializationFailed(m));
        }
    };

    let generic_struct = parsing::parsed_to_generic_struct(ron_struct, options, ron_to_raw_value);

    Ok(generic_struct)
}

#[allow(float_cmp)]
fn ron_to_raw_value(
    super_struct: &str,
    super_key: &str,
    value: Value,
    options: &Options,
) -> GenericValue {
    match value {
        Value::Unit => GenericValue::Unit,
        Value::Bool(value) => GenericValue::Bool(value),
        Value::Char(value) => GenericValue::Char(value),
        Value::Number(value) => {
            let float = value.get();

            if float.trunc() == float {
                parsing::preferred_int(float as i64, options.default_int_size)
            } else {
                parsing::preferred_float(float, options.default_float_size)
            }
        }
        Value::String(value) => GenericValue::String(value),
        Value::Option(option) => GenericValue::Option(
            option
                .map(|value| Box::new(ron_to_raw_value(super_struct, super_key, *value, options))),
        ),
        Value::Seq(values) => GenericValue::Array(
            values
                .into_iter()
                .map(|value| ron_to_raw_value(super_struct, super_key, value, options))
                .collect(),
        ),
        Value::Map(values) => {
            let sub_struct_name = format!("{}__{}", super_struct, super_key);
            let values = values
                .into_iter()
                .map(|(key, value)| {
                    let key = {
                        if let Value::String(key) = key {
                            key
                        } else {
                            unimplemented!("We should handle an error here");
                        }
                    };
                    let value = ron_to_raw_value(&sub_struct_name, &key, value, options);
                    (key, value)
                })
                .collect();
            GenericValue::Struct(GenericStruct {
                struct_name: sub_struct_name,
                fields: values,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_string_keys() {
        let ron_code = r#"(100: "One hundred")"#;
        assert!(parse_ron(ron_code, &Options::default()).is_err());
    }

    #[test]
    fn test_non_struct_root_object() {
        let ron_code = r#"["key", "value"]"#;
        assert!(parse_ron(ron_code, &Options::default()).is_err());
    }
}
