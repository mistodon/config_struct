#[macro_use]
extern crate serde_derive;

extern crate toml;

use std::collections::HashMap;
use std::path::Path;

use toml::Value;


type RawConfig = HashMap<String, RawValue>;


enum RawValue
{
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Isize(isize),
    Usize(usize),
    F32(f32),
    F64(f64),
    String(String),
    Array(Vec<RawValue>),
    Struct(String, HashMap<String, RawValue>)
}


pub fn construct_config<S, D>(config_filename: S, destination_filename: D)
where
    S: AsRef<Path>,
    D: AsRef<Path>
{
    use std::fs::File;
    use std::io::{ Read, Write };

    let config_source = {
        let mut buffer = String::new();
        let file = &mut File::open(&config_filename).unwrap();
        file.read_to_string(&mut buffer).unwrap();
        buffer
    };

    let toml_object: HashMap<String, Value> = toml::from_str(&config_source).unwrap();

    let raw_config: RawConfig = toml_object.into_iter().map(|(key, value)|
    {
        (key, toml_to_raw_value(value))
    }).collect();

    let config_rust_code = {
        let mut code = String::new();

        code.push_str(
"use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
");

        for (field_name, value) in raw_config.iter()
        {
            let field_type = type_string(value);
            code.push_str(&format!("    pub {}: {},\n", field_name, field_type));
        }

        code.push_str(
"}

pub const CONFIG: Config = Config {
");

        for (field_name, value) in raw_config.iter()
        {
            let field_value = value_string(value);
            code.push_str(&format!("    {}: {},\n", field_name, field_value));
        }

        code.push_str(
"};
");

        code
    };

    let destination_file = &mut File::create(destination_filename).unwrap();
    destination_file.write_all(config_rust_code.as_bytes()).unwrap();
}


fn toml_to_raw_value(value: Value) -> RawValue
{
    match value
    {
        Value::Boolean(value) => RawValue::Bool(value),
        Value::Integer(value) => RawValue::I64(value),
        Value::Float(value) => RawValue::F64(value),
        Value::String(value) => RawValue::String(value),
        Value::Datetime(value) => RawValue::String(value.to_string()),
        Value::Array(values) =>
            RawValue::Array(values.into_iter().map(toml_to_raw_value).collect()),
        Value::Table(value) => unimplemented!()
    }
}


fn type_string(value: &RawValue) -> String
{
    match *value
    {
        RawValue::Bool(_) => "bool".to_owned(),
        RawValue::I8(_) => "i8".to_owned(),
        RawValue::I16(_) => "i16".to_owned(),
        RawValue::I32(_) => "i32".to_owned(),
        RawValue::I64(_) => "i64".to_owned(),
        RawValue::U8(_) => "u8".to_owned(),
        RawValue::U16(_) => "u16".to_owned(),
        RawValue::U32(_) => "u32".to_owned(),
        RawValue::U64(_) => "u64".to_owned(),
        RawValue::Isize(_) => "isize".to_owned(),
        RawValue::Usize(_) => "usize".to_owned(),
        RawValue::F32(_) => "f32".to_owned(),
        RawValue::F64(_) => "f64".to_owned(),
        RawValue::String(_) => "Cow<'static, str>".to_owned(),
        RawValue::Array(ref values) => {
            assert!(!values.is_empty());
            let candidate = type_string(&values[0]);
            let all_same_type = values.iter()
                .map(type_string)
                .all(|s| s == candidate);
            assert!(all_same_type);
            format!("Cow<'static, [{}]>", candidate)
        },
        RawValue::Struct(_, _) => unimplemented!(),
    }
}


fn value_string(value: &RawValue) -> String
{
    match *value
    {
        RawValue::Bool(value) => value.to_string(),
        RawValue::I8(value) => value.to_string(),
        RawValue::I16(value) => value.to_string(),
        RawValue::I32(value) => value.to_string(),
        RawValue::I64(value) => value.to_string(),
        RawValue::U8(value) => value.to_string(),
        RawValue::U16(value) => value.to_string(),
        RawValue::U32(value) => value.to_string(),
        RawValue::U64(value) => value.to_string(),
        RawValue::Isize(value) => value.to_string(),
        RawValue::Usize(value) => value.to_string(),
        RawValue::F32(value) => float_string(value),
        RawValue::F64(value) => float_string(value),
        RawValue::String(ref value) => format!("Cow::Borrowed(\"{}\")", value),
        RawValue::Array(ref values) => {
            let value_strings = values.iter().map(value_string).collect::<Vec<String>>();
            format!("Cow::Borrowed(&[{}])", value_strings.join(", "))
        },
        RawValue::Struct(_, _) => unimplemented!(),
    }
}


fn float_string<T>(float: T) -> String
where
    T: ToString
{
    let mut result = float.to_string();
    if !result.contains('.')
    {
        result.push_str(".0");
    }
    result
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn float_string_tests()
    {
        assert_eq!(float_string(1.0), "1.0");
        assert_eq!(float_string(1.5), "1.5");
        assert_eq!(float_string(-2.5), "-2.5");
        assert_eq!(float_string(123.456789), "123.456789");
    }


    fn test_array<F, T>(f: F, vec: Vec<T>) -> RawValue
    where
        F: Fn(T) -> RawValue
    {
        RawValue::Array(vec.into_iter().map(|x| f(x)).collect())
    }

    #[test]
    fn simple_array_type_string_tests()
    {
        assert_eq!(type_string(&test_array(RawValue::F32, vec![1.0, 2.0])), "Cow<'static, [f32]>");
        assert_eq!(type_string(&test_array(RawValue::I32, vec![1, -2])), "Cow<'static, [i32]>");
        assert_eq!(
            type_string(&test_array(RawValue::String, vec!["one".to_owned(), "two".to_owned()])),
            "Cow<'static, [Cow<'static, str>]>");
    }

    #[test]
    fn simple_array_value_strings_tests()
    {
        assert_eq!(value_string(&test_array(RawValue::I32, vec![1])), "Cow::Borrowed(&[1])");
        assert_eq!(value_string(&test_array(RawValue::I32, vec![1, 3])), "Cow::Borrowed(&[1, 3])");
        assert_eq!(value_string(&test_array(RawValue::I32, vec![1, 3, 5])), "Cow::Borrowed(&[1, 3, 5])");
        assert_eq!(
            value_string(&test_array(RawValue::String, vec!["one".to_owned(), "two".to_owned()])),
            "Cow::Borrowed(&[Cow::Borrowed(\"one\"), Cow::Borrowed(\"two\")])");
    }
}

