use serde_json::{self, Value};

use error::GenerationError;
use options::Options;
use parsing;
use value::{GenericStruct, GenericValue};

pub fn parse_json(
    json: &str,
    options: &Options,
) -> Result<GenericStruct, GenerationError> {
    use parsing::ParsedFields;

    let json_struct: ParsedFields<Value> = serde_json::from_str(json)
        .map_err(|err| GenerationError::DeserializationFailed(err.to_string()))?;

    let generic_struct = parsing::parsed_to_generic_struct(
        json_struct, json_to_raw_value);

    Ok(generic_struct)
}

fn json_to_raw_value(
    super_struct: &str,
    super_key: &str,
    value: Value,
) -> GenericValue {
    match value {
        Value::Null => GenericValue::Option(None),
        Value::Bool(value) => GenericValue::Bool(value),
        Value::Number(value) => match (value.as_i64(), value.as_u64(), value.as_f64()) {
            (Some(x), _, _) => GenericValue::I64(x),
            (None, Some(x), _) => GenericValue::U64(x),
            (None, None, Some(x)) => GenericValue::F64(x),
            _ => unimplemented!("Should handle error here"),
        },
        Value::String(value) => GenericValue::String(value),
        Value::Array(values) => GenericValue::Array(
            values
                .into_iter()
                .map(|value| json_to_raw_value(super_struct, super_key, value))
                .collect(),
        ),
        Value::Object(values) => {
            let sub_struct_name = format!("{}__{}", super_struct, super_key);
            let values = values
                .into_iter()
                .map(|(key, value)| {
                    let value = json_to_raw_value(&sub_struct_name, &key, value);
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
