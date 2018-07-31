use toml::{self, Value};

use error::GenerationError;
use options::Options;
use parsing;
use value::{GenericStruct, GenericValue};

pub fn parse_toml(toml: &str, _options: &Options) -> Result<GenericStruct, GenerationError> {
    use parsing::ParsedFields;

    let toml_struct: ParsedFields<Value> = toml::from_str(toml)
        .map_err(|err| GenerationError::DeserializationFailed(err.to_string()))?;

    let generic_struct = parsing::parsed_to_generic_struct(toml_struct, toml_to_raw_value);

    Ok(generic_struct)
}

fn toml_to_raw_value(super_struct: &str, super_key: &str, value: Value) -> GenericValue {
    match value {
        Value::Boolean(value) => GenericValue::Bool(value),
        Value::Integer(value) => GenericValue::I64(value),
        Value::Float(value) => GenericValue::F64(value),
        Value::String(value) => GenericValue::String(value),
        Value::Datetime(value) => GenericValue::String(value.to_string()),
        Value::Array(values) => GenericValue::Array(
            values
                .into_iter()
                .map(|value| toml_to_raw_value(super_struct, super_key, value))
                .collect(),
        ),
        Value::Table(values) => {
            let sub_struct_name = format!("{}__{}", super_struct, super_key);
            let values = values
                .into_iter()
                .map(|(key, value)| {
                    let value = toml_to_raw_value(&sub_struct_name, &key, value);
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
