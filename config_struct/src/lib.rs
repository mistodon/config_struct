extern crate toml;

use std::collections::BTreeMap;
use std::path::Path;

use toml::Value;


pub struct RawStructValue
{
    pub struct_name: String,
    pub fields: BTreeMap<String, RawValue>
}

pub enum RawValue
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
    Struct(RawStructValue)
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

    let toml_object: BTreeMap<String, Value> = toml::from_str(&config_source).unwrap();

    let raw_config: RawStructValue = {
        let struct_name = "Config".to_owned();
        let fields = toml_object.into_iter().map(|(key, value)|
        {
            let value = toml_to_raw_value("_Config", &key, value);
            (key, value)
        }).collect();
        RawStructValue { struct_name, fields }
    };

    let config_rust_code = {
        let mut code = String::new();

        code.push_str("use std::borrow::Cow;\n\n");

        generate_struct_declarations(&mut code, &raw_config);

        let raw_config_as_value = RawValue::Struct(raw_config);
        code.push_str(
            &format!(
                "pub const CONFIG: Config = {};\n",
                value_string(&raw_config_as_value, 0)));

        code
    };

    let destination_file = &mut File::create(destination_filename).unwrap();
    destination_file.write_all(config_rust_code.as_bytes()).unwrap();
}


fn toml_to_raw_value(super_struct: &str, super_key: &str, value: Value) -> RawValue
{
    match value
    {
        Value::Boolean(value) => RawValue::Bool(value),
        Value::Integer(value) => RawValue::I64(value),
        Value::Float(value) => RawValue::F64(value),
        Value::String(value) => RawValue::String(value),
        Value::Datetime(value) => RawValue::String(value.to_string()),
        Value::Array(values) => {
            RawValue::Array(values.into_iter()
                .map(|value| toml_to_raw_value(super_struct, super_key, value))
                .collect())
        },
        Value::Table(values) => {
            let sub_struct_name = format!("{}__{}", super_struct, super_key);
            let values = values.into_iter()
                .map(
                    |(key, value)|
                    {
                        let value = toml_to_raw_value(&sub_struct_name, &key, value);
                        (key, value)
                    })
                .collect();
            RawValue::Struct(RawStructValue { struct_name: sub_struct_name, fields: values })
        }
    }
}


fn generate_struct_declarations(output: &mut String, struct_value: &RawStructValue)
{
    let field_strings = struct_value.fields.iter()
        .map(|(name, value)| format!("    pub {}: {},", name, type_string(value)))
        .collect::<Vec<String>>();
    output.push_str(&format!(
"#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct {} {{
{}
}}

", struct_value.struct_name, field_strings.join("\n")));

    for (_, value) in &struct_value.fields
    {
        if let &RawValue::Struct(ref struct_value) = value
        {
            generate_struct_declarations(output, struct_value);
        }
        else if let &RawValue::Array(ref values) = value
        {
            if let RawValue::Struct(ref struct_value) = values[0]
            {
                generate_struct_declarations(output, struct_value);
            }
        }
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
        RawValue::Struct(ref struct_value) => struct_value.struct_name.clone(),
    }
}


fn value_string(value: &RawValue, indentation: usize) -> String
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
            let value_strings = values.iter().map(|value| value_string(value, indentation + 4)).collect::<Vec<String>>();
            format!("Cow::Borrowed(&[{}])", value_strings.join(", "))
        },
        RawValue::Struct(ref struct_value) => {
            let values = struct_value.fields.iter()
                .map(|(field, value)| format!("{:indent$}{}: {},\n", "", field, value_string(value, indentation + 4), indent = indentation + 4))
                .collect::<Vec<String>>();
            format!("{} {{\n{}{:indent$}}}", struct_value.struct_name, values.join(""), "", indent = indentation)
        },
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

