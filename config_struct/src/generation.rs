use options::{ Options };
use value::{ RawValue, RawStructValue };


pub fn generate_structs(
    struct_value: &RawStructValue,
    options: &Options) -> String
{
    let mut buffer = String::new();
    generate_struct_declarations(&mut buffer, struct_value, options);
    buffer
}


fn generate_struct_declarations(
    output: &mut String,
    struct_value: &RawStructValue,
    options: &Options)
{
    let field_strings = struct_value.fields.iter()
        .map(|(name, value)| format!("    pub {}: {},", name, type_string(value)))
        .collect::<Vec<String>>();

    let derive_string = {
        if options.derived_traits.is_empty()
        {
            "".to_owned()
        }
        else
        {
            format!("#[derive({})]\n", options.derived_traits.join(", "))
        }
    };

    output.push_str(&format!(
"{}#[allow(non_camel_case_types)]
pub struct {} {{
{}
}}

", derive_string, struct_value.struct_name, field_strings.join("\n")));

    // TODO: is this ... accurate? Does this handle nested arrays/options???
    for value in struct_value.fields.values()
    {
        match *value
        {
            RawValue::Struct(ref value) => generate_struct_declarations(output, value, options),
            RawValue::Array(ref values) => {
                if let Some(&RawValue::Struct(ref value)) = values.get(0)
                {
                    generate_struct_declarations(output, value, options);
                }
            }
            _ => ()
        }
    }
}


// TODO: Shouldn't really need to be public
pub fn type_string(value: &RawValue) -> String
{
    match *value
    {
        RawValue::Unit => "()".to_owned(),
        RawValue::Bool(_) => "bool".to_owned(),
        RawValue::Char(_) => "char".to_owned(),
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
        RawValue::Option(ref value) => {
            let element_type = match *value
            {
                Some(ref value) => type_string(value),
                None => type_string(&RawValue::Unit)
            };
            format!("Option<{}>", element_type)
        },
        RawValue::Array(ref values) => {
            let element_type = match values.get(0)
            {
                Some(element) => type_string(element),
                None => type_string(&RawValue::Unit)
            };
            format!("Cow<'static, [{}]>", element_type)
        },
        RawValue::Struct(ref struct_value) => struct_value.struct_name.clone(),
    }
}


fn value_string(value: &RawValue, indentation: usize) -> String
{
    match *value
    {
        RawValue::Unit => "()".to_string(),
        RawValue::Bool(value) => value.to_string(),
        RawValue::Char(value) => format!("'{}'", value),
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
        RawValue::Option(ref value) => match *value
        {
            Some(ref value) => format!("Some({})", value_string(value, indentation)),
            None => "None".to_string()
        },
        RawValue::Array(ref values) => {
            let value_strings = values.iter().map(|value| value_string(value, indentation + 4)).collect::<Vec<String>>();
            format!("Cow::Borrowed(&[{}])", value_strings.join(", "))
        },
        RawValue::Struct(ref struct_value) => struct_value_string(struct_value, indentation),
    }
}

pub fn struct_value_string(value: &RawStructValue, indentation: usize) -> String
{
    let values = value.fields.iter()
        .map(|(field, value)| format!("{:indent$}{}: {},\n", "", field, value_string(value, indentation + 4), indent = indentation + 4))
        .collect::<Vec<String>>();
    format!("{} {{\n{}{:indent$}}}", value.struct_name, values.join(""), "", indent = indentation)
}


fn float_string<T>(float: T) -> String
where
    T: ToString + Copy
{
    let mut result = float.to_string();
    if !result.contains('.')
    {
        result.push_str(".0");
    }
    result
}

