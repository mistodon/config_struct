use serde_yaml::{self, Value};

use crate::{
    error::GenerationError,
    options::StructOptions,
    parsing,
    value::{GenericStruct, GenericValue},
};

pub fn parse_yaml(yaml: &str, options: &StructOptions) -> Result<GenericStruct, GenerationError> {
    use parsing::ParsedFields;

    let yaml_struct: ParsedFields<Value> = serde_yaml::from_str(yaml)
        .map_err(|err| GenerationError::DeserializationFailed(err.to_string()))?;

    let generic_struct = parsing::parsed_to_generic_struct(yaml_struct, options, yaml_to_raw_value);

    Ok(generic_struct)
}

fn yaml_to_raw_value(
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
        Value::Sequence(values) => GenericValue::Array(
            values
                .into_iter()
                .map(|value| yaml_to_raw_value(super_struct, super_key, value, options))
                .collect(),
        ),
        Value::Mapping(values) => {
            let sub_struct_name = format!("{}__{}", super_struct, super_key);
            let values = values
                .into_iter()
                .map(|(key, value)| {
                    let key = key.as_str().unwrap().to_owned();
                    let value = yaml_to_raw_value(&sub_struct_name, &key, value, options);
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

pub fn parse_map_keys(yaml: &str) -> Result<Vec<String>, GenerationError> {
    use linear_map::LinearMap;

    let map: LinearMap<String, Value> = serde_yaml::from_str(yaml).map_err(|err| GenerationError::DeserializationFailed(err.to_string()))?;

    Ok(map.into_iter().map(|pair| pair.0).collect())
}
