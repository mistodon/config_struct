use options::Options;
use value::{GenericStruct, GenericValue};

pub fn generate_structs(struct_value: &GenericStruct, options: &Options) -> String {
    let mut buffer = String::new();
    generate_struct_declarations(&mut buffer, struct_value, options);
    buffer
}

fn generate_struct_declarations(
    output: &mut String,
    struct_value: &GenericStruct,
    options: &Options,
) {
    let field_strings = struct_value
        .fields
        .iter()
        .map(|(name, value)| {
            format!(
                "    pub {}: {},",
                name,
                type_string_with_options(value, options.max_array_size)
            )
        })
        .collect::<Vec<String>>();

    let derive_string = {
        if options.derived_traits.is_empty() {
            "".to_owned()
        } else {
            format!("#[derive({})]\n", options.derived_traits.join(", "))
        }
    };

    output.push_str(&format!(
        "{}#[allow(non_camel_case_types)]
pub struct {} {{
{}
}}

",
        derive_string,
        struct_value.struct_name,
        field_strings.join("\n")
    ));

    // TODO: is this ... accurate? Does this handle nested arrays/options???
    for value in struct_value.fields.values() {
        match *value {
            GenericValue::Struct(ref value) => generate_struct_declarations(output, value, options),
            GenericValue::Array(ref values) => {
                if let Some(&GenericValue::Struct(ref value)) = values.get(0) {
                    generate_struct_declarations(output, value, options);
                }
            }
            _ => (),
        }
    }
}

// TODO: Shouldn't really need to be public
pub fn type_string(value: &GenericValue) -> String {
    type_string_with_options(value, 0)
}

// TODO: So ugly, wow.
fn type_string_with_options(value: &GenericValue, max_array_size: usize) -> String {
    match *value {
        GenericValue::Unit => "()".to_owned(),
        GenericValue::Bool(_) => "bool".to_owned(),
        GenericValue::Char(_) => "char".to_owned(),
        GenericValue::I8(_) => "i8".to_owned(),
        GenericValue::I16(_) => "i16".to_owned(),
        GenericValue::I32(_) => "i32".to_owned(),
        GenericValue::I64(_) => "i64".to_owned(),
        GenericValue::U8(_) => "u8".to_owned(),
        GenericValue::U16(_) => "u16".to_owned(),
        GenericValue::U32(_) => "u32".to_owned(),
        GenericValue::U64(_) => "u64".to_owned(),
        GenericValue::ISize(_) => "isize".to_owned(),
        GenericValue::Usize(_) => "usize".to_owned(),
        GenericValue::F32(_) => "f32".to_owned(),
        GenericValue::F64(_) => "f64".to_owned(),
        GenericValue::String(_) => "Cow<'static, str>".to_owned(),
        GenericValue::Option(ref value) => {
            let element_type = match *value {
                Some(ref value) => type_string_with_options(value, max_array_size),
                None => type_string_with_options(&GenericValue::Unit, max_array_size),
            };
            format!("Option<{}>", element_type)
        }
        GenericValue::Array(ref values) => {
            let element_type = match values.get(0) {
                Some(element) => type_string_with_options(element, max_array_size),
                None => type_string_with_options(&GenericValue::Unit, max_array_size),
            };
            if !values.is_empty() && values.len() <= max_array_size {
                format!("[{}; {}]", element_type, values.len())
            } else {
                format!("Cow<'static, [{}]>", element_type)
            }
        }
        GenericValue::Struct(ref struct_value) => struct_value.struct_name.clone(),
    }
}

fn value_string(value: &GenericValue, indentation: usize, max_array_size: usize) -> String {
    match *value {
        GenericValue::Unit => "()".to_string(),
        GenericValue::Bool(value) => value.to_string(),
        GenericValue::Char(value) => format!("'{}'", value),
        GenericValue::I8(value) => value.to_string(),
        GenericValue::I16(value) => value.to_string(),
        GenericValue::I32(value) => value.to_string(),
        GenericValue::I64(value) => value.to_string(),
        GenericValue::U8(value) => value.to_string(),
        GenericValue::U16(value) => value.to_string(),
        GenericValue::U32(value) => value.to_string(),
        GenericValue::U64(value) => value.to_string(),
        GenericValue::ISize(value) => value.to_string(),
        GenericValue::Usize(value) => value.to_string(),
        GenericValue::F32(value) => float_string(value),
        GenericValue::F64(value) => float_string(value),
        GenericValue::String(ref value) => format!("Cow::Borrowed(\"{}\")", value),
        GenericValue::Option(ref value) => match *value {
            Some(ref value) => {
                format!("Some({})", value_string(value, indentation, max_array_size))
            }
            None => "None".to_string(),
        },
        GenericValue::Array(ref values) => {
            let value_strings = values
                .iter()
                .map(|value| value_string(value, indentation + 4, max_array_size))
                .collect::<Vec<String>>();

            if !values.is_empty() && values.len() <= max_array_size {
                format!("[{}]", value_strings.join(", "))
            } else {
                format!("Cow::Borrowed(&[{}])", value_strings.join(", "))
            }
        }
        GenericValue::Struct(ref struct_value) => {
            struct_value_string(struct_value, indentation, max_array_size)
        }
    }
}

pub fn struct_value_string(
    value: &GenericStruct,
    indentation: usize,
    max_array_size: usize,
) -> String {
    let values = value
        .fields
        .iter()
        .map(|(field, value)| {
            format!(
                "{:indent$}{}: {},\n",
                "",
                field,
                value_string(value, indentation + 4, max_array_size),
                indent = indentation + 4
            )
        })
        .collect::<Vec<String>>();
    format!(
        "{} {{\n{}{:indent$}}}",
        value.struct_name,
        values.join(""),
        "",
        indent = indentation
    )
}

fn float_string<T>(float: T) -> String
where
    T: ToString + Copy,
{
    let mut result = float.to_string();
    if !result.contains('.') {
        result.push_str(".0");
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const U: GenericValue = GenericValue::Unit;

    #[test]
    fn empty_slice_type() {
        assert_eq!(
            type_string_with_options(&GenericValue::Array(vec![]), 0),
            "Cow<'static, [()]>",
        );

        assert_eq!(
            type_string_with_options(&GenericValue::Array(vec![]), 4),
            "Cow<'static, [()]>",
        );
    }

    #[test]
    fn non_empty_slice_type() {
        assert_eq!(
            type_string_with_options(&GenericValue::Array(vec![U, U, U]), 0),
            "Cow<'static, [()]>",
        );
    }

    #[test]
    fn non_empty_array_type() {
        assert_eq!(
            type_string_with_options(&GenericValue::Array(vec![U, U, U]), 4),
            "[(); 3]",
        );
    }

    #[test]
    fn empty_slice_value() {
        assert_eq!(
            value_string(&GenericValue::Array(vec![]), 0, 0),
            "Cow::Borrowed(&[])",
        );

        assert_eq!(
            value_string(&GenericValue::Array(vec![]), 0, 4),
            "Cow::Borrowed(&[])",
        );
    }

    #[test]
    fn non_empty_slice_value() {
        assert_eq!(
            value_string(&GenericValue::Array(vec![U, U, U]), 0, 0),
            "Cow::Borrowed(&[(), (), ()])",
        );
    }

    #[test]
    fn non_empty_array_value() {
        assert_eq!(
            value_string(&GenericValue::Array(vec![U, U, U]), 0, 4),
            "[(), (), ()]",
        );
    }
}
