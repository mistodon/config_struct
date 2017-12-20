use generation;
use value::{ RawStructValue, RawValue };


#[derive(Debug, Fail)]
pub enum StructGenerationError
{
    #[fail(display = "config contains a heterogenous array - all array elements must have the same fields")]
    HeterogenousArray,

    #[fail(display = "config contains an invalid key: \"{}\" - all keys must be valid Rust identifiers", key_name)]
    InvalidFieldName
    {
        key_name: String
    }
}


pub fn validate_struct_value(struct_value: &RawStructValue) -> Result<(), StructGenerationError>
{
    for (key, value) in &struct_value.fields
    {
        validate_field_name(key)?;
        validate_value(value)?;
    }
    Ok(())
}


fn validate_field_name(field_name: &str) -> Result<(), StructGenerationError>
{
    use std::ascii::AsciiExt;

    let good_start = field_name.starts_with(|c: char| c == '_' || (c.is_ascii() && c.is_alphabetic()));
    let good_end = !field_name.contains(|c: char| !(c == '_' || c.is_digit(10) || (c.is_ascii() && c.is_alphabetic())));

    if good_start && good_end && field_name != "_"
    {
        Ok(())
    }
    else
    {
        Err(StructGenerationError::InvalidFieldName { key_name: field_name.to_owned() })
    }
}


fn validate_value(value: &RawValue) -> Result<(), StructGenerationError>
{
    match *value
    {
        RawValue::Option(Some(ref value)) => validate_value(value)?,
        RawValue::Array(ref values) => {
            validate_array_element_types(values)?;
            for value in values
            {
                validate_value(value)?;
            };
        },
        RawValue::Struct(ref value) => validate_struct_value(value)?,
        _ => ()
    }
    Ok(())
}


fn validate_array_element_types(values: &[RawValue]) -> Result<(), StructGenerationError>
{
    if let Some(ref value) = values.get(0)
    {
        // TODO: A more efficient way to compare types would be nice
        let candidate = generation::type_string(value);
        let all_same_type = values.iter()
            .map(generation::type_string)
            .all(|s| s == candidate);

        if !all_same_type
        {
            return Err(StructGenerationError::HeterogenousArray);
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn valid_field_names()
    {
        assert!(validate_field_name("x").is_ok());
        assert!(validate_field_name("word").is_ok());
        assert!(validate_field_name("two_words").is_ok());
        assert!(validate_field_name("PascalCase").is_ok());
        assert!(validate_field_name("number_150").is_ok());
        assert!(validate_field_name("_private").is_ok());
        assert!(validate_field_name("____very_private__").is_ok());
    }

    #[test]
    fn invalid_field_names()
    {
        assert!(validate_field_name("").is_err());
        assert!(validate_field_name("_").is_err());
        assert!(validate_field_name("100_number_before").is_err());
        assert!(validate_field_name("white space").is_err());
        assert!(validate_field_name("wierd*characters??").is_err());
        assert!(validate_field_name("emoji😇").is_err());
        assert!(validate_field_name("accénts").is_err());
    }

    #[test]
    fn homogenous_arrays()
    {
        assert!(validate_array_element_types(&[]).is_ok());
        assert!(validate_array_element_types(&[RawValue::Unit]).is_ok());
        assert!(validate_array_element_types(&[RawValue::Unit, RawValue::Unit]).is_ok());
        assert!(validate_array_element_types(&[RawValue::I64(0), RawValue::I64(1)]).is_ok());
        assert!(validate_array_element_types(&[
            RawValue::Array(vec![RawValue::Unit]),
            RawValue::Array(vec![RawValue::Unit])]).is_ok());
    }

    #[test]
    fn heterogenous_arrays()
    {
        assert!(validate_array_element_types(&[RawValue::Unit, RawValue::I64(0)]).is_err());
        assert!(validate_array_element_types(&[
            RawValue::Array(vec![RawValue::Unit]),
            RawValue::Array(vec![RawValue::I64(0)])]).is_err());
    }
}
