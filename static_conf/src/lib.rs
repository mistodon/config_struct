#[macro_use]
extern crate serde_derive;

extern crate toml;

use std::collections::HashMap;
use std::path::Path;


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
    use toml::Value;

    let config_source = {
        let mut buffer = String::new();
        let file = &mut File::open(&config_filename).unwrap();
        file.read_to_string(&mut buffer).unwrap();
        buffer
    };

    let toml_object: HashMap<String, Value> = toml::from_str(&config_source).unwrap();

    let raw_config: RawConfig = toml_object.into_iter().map(|(key, value)|
    {
        let value = match value
        {
            Value::String(value) => RawValue::String(value),
            _ => unimplemented!()
        };
        (key, value)
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
            code.push_str(&format!("    pub {}: {}\n", field_name, field_type));
        }

        code.push_str(
"}

pub const CONFIG: Config = Config {
");

        for (field_name, value) in raw_config.iter()
        {
            let field_value = value_string(value);
            code.push_str(&format!("    {}: {}\n", field_name, field_value));
        }

        code.push_str(
"};
");

        code
    };

    let destination_file = &mut File::create(destination_filename).unwrap();
    destination_file.write_all(config_rust_code.as_bytes()).unwrap();
}


fn type_string(value: &RawValue) -> String
{
    match *value
    {
        RawValue::String(ref value) => "Cow<'static, str>".to_owned(),
        _ => unimplemented!()
    }
}


fn value_string(value: &RawValue) -> String
{
    match *value
    {
        RawValue::String(ref value) => format!("Cow::Borrowed(\"{}\")", value),
        _ => unimplemented!()
    }
}
