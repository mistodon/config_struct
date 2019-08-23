use serde_json::{self, Value};

use crate::{
    error::GenerationError,
    options::StructOptions,
    parsing,
    value::{GenericStruct, GenericValue},
};

pub fn parse_json(json: &str, options: &StructOptions) -> Result<GenericStruct, GenerationError> {
    use parsing::ParsedFields;

    let json_struct: ParsedFields<Value> = serde_json::from_str(json)
        .map_err(|err| GenerationError::DeserializationFailed(err.to_string()))?;

    let generic_struct = parsing::parsed_to_generic_struct(json_struct, options, json_to_raw_value);

    Ok(generic_struct)
}

fn json_to_raw_value(
    super_struct: &str,
    super_key: &str,
    value: Value,
    options: &StructOptions,
) -> GenericValue {
    match value {
        Value::Null => GenericValue::Option(None),
        Value::Bool(value) => GenericValue::Bool(value),
        Value::Number(value) => match (value.as_i64(), value.as_u64(), value.as_f64()) {
            // TODO: Add some unit tests for this
            (Some(x), _, _) => parsing::preferred_int(x, options.default_int_size),
            (None, Some(x), _) => GenericValue::U64(x),
            (None, None, Some(x)) => parsing::preferred_float(x, options.default_float_size),
            _ => unimplemented!("Should handle error here"), // TODO
        },
        Value::String(value) => GenericValue::String(value),
        Value::Array(values) => GenericValue::Array(
            values
                .into_iter()
                .map(|value| json_to_raw_value(super_struct, super_key, value, options))
                .collect(),
        ),
        Value::Object(values) => {
            let sub_struct_name = format!("{}__{}", super_struct, super_key);
            let values = values
                .into_iter()
                .map(|(key, value)| {
                    let value = json_to_raw_value(&sub_struct_name, &key, value, options);
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
