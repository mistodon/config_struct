use crate::{
    error::GenerationError,
    generation::type_string, // TODO: I wish we weren't comparing types by string,
    value::{GenericStruct, GenericValue},
};

pub fn valid_identifier(name: &str) -> bool {
    let good_start = name.starts_with(|c: char| c == '_' || (c.is_ascii() && c.is_alphabetic()));
    let good_end = !name
        .contains(|c: char| !(c == '_' || c.is_digit(10) || (c.is_ascii() && c.is_alphabetic())));

    good_start && good_end && name != "_"
}

pub fn validate_struct(struct_value: &GenericStruct) -> Result<(), GenerationError> {
    for (key, value) in &struct_value.fields {
        validate_field_name(key)?;
        validate_value(key, value)?;
    }
    Ok(())
}

fn validate_field_name(field_name: &str) -> Result<(), GenerationError> {
    if valid_identifier(field_name) {
        Ok(())
    } else {
        Err(GenerationError::InvalidFieldName(field_name.into()))
    }
}

fn validate_value(key: &str, value: &GenericValue) -> Result<(), GenerationError> {
    match *value {
        GenericValue::Option(Some(ref value)) => validate_value(key, value)?,
        GenericValue::Array(ref values) => {
            validate_array_element_types(key, values)?;
            for value in values {
                validate_value(key, value)?;
            }
        }
        GenericValue::Struct(ref value) => validate_struct(value)?,
        _ => (),
    }
    Ok(())
}

fn validate_array_element_types(key: &str, values: &[GenericValue]) -> Result<(), GenerationError> {
    if let Some(ref value) = values.get(0) {
        // TODO: A more efficient way to compare types would be nice
        let candidate = type_string(value);
        let all_same_type = values.iter().map(type_string).all(|s| s == candidate);

        if !all_same_type {
            return Err(GenerationError::HeterogenousArray(key.into()));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_field_names() {
        assert_eq!(valid_identifier("x"), true);
        assert_eq!(valid_identifier("word"), true);
        assert_eq!(valid_identifier("two_words"), true);
        assert_eq!(valid_identifier("PascalCase"), true);
        assert_eq!(valid_identifier("number_150"), true);
        assert_eq!(valid_identifier("_private"), true);
        assert_eq!(valid_identifier("____very_private__"), true);
    }

    #[test]
    fn invalid_field_names() {
        assert_eq!(valid_identifier(""), false);
        assert_eq!(valid_identifier("_"), false);
        assert_eq!(valid_identifier("100_number_before"), false);
        assert_eq!(valid_identifier("white space"), false);
        assert_eq!(valid_identifier("wierd*characters??"), false);
        assert_eq!(valid_identifier("emoji😇"), false);
        assert_eq!(valid_identifier("accénts"), false);
    }

    fn validate_array_test(values: &[GenericValue]) -> Result<(), GenerationError> {
        validate_array_element_types("", values)
    }

    #[test]
    fn homogenous_arrays() {
        assert!(validate_array_test(&[]).is_ok());
        assert!(validate_array_test(&[GenericValue::Unit]).is_ok());
        assert!(validate_array_test(&[GenericValue::Unit, GenericValue::Unit]).is_ok());
        assert!(validate_array_test(&[GenericValue::I64(0), GenericValue::I64(1)]).is_ok());
        assert!(validate_array_test(&[
            GenericValue::Array(vec![GenericValue::Unit]),
            GenericValue::Array(vec![GenericValue::Unit]),
        ])
        .is_ok());
    }

    #[test]
    fn heterogenous_arrays() {
        assert!(validate_array_test(&[GenericValue::Unit, GenericValue::I64(0)]).is_err());
        assert!(validate_array_test(&[
            GenericValue::Array(vec![GenericValue::Unit]),
            GenericValue::Array(vec![GenericValue::I64(0)]),
        ])
        .is_err());
    }
}
