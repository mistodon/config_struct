use crate::{
    options::{EnumOptions, StructOptions},
    value::{GenericStruct, GenericValue},
};

pub fn generate_structs(struct_value: &GenericStruct, options: &StructOptions) -> String {
    let mut buffer = String::new();
    generate_struct_declarations(&mut buffer, struct_value, options);
    buffer
}

pub fn generate_enum(variants: &[String], options: &EnumOptions) -> String {
    use quote::{format_ident, quote};

    let first_variant = variants.get(0).map(|name| format_ident!("{}", name));
    let keys = variants.iter().map(|name| format_ident!("{}", name));
    let const_keys = keys.clone();
    let string_keys = variants.iter();
    let enum_name = format_ident!("{}", options.enum_name);
    let const_name = options
        .all_variants_const
        .as_ref()
        .map(|name| format_ident!("{}", name));
    let from_str_const_name = const_name.clone();

    // TODO: This is not robust to more complex derives
    // (with package name for example)
    let derive_tokens = {
        let mut derives = options
            .derived_traits
            .iter()
            .map(|name| {
                let token = format_ident!("{}", name);
                quote! { #token }
            })
            .collect::<Vec<_>>();

        if let Some((ser, de)) = options.serde_support.should_derive_ser_de() {
            let prefix = if options.use_serde_derive_crate {
                format_ident!("serde_derive")
            } else {
                format_ident!("serde")
            };

            if ser {
                derives.push(quote! { #prefix::Serialize });
            }
            if de {
                derives.push(quote! { #prefix::Deserialize });
            }
        }

        if derives.is_empty() {
            quote! {}
        } else {
            quote! {
                #[derive(#(#derives),*)]
            }
        }
    };

    let mut tokens = quote! {
        #![cfg_attr(rustfmt, rustfmt_skip)]
        #![allow(dead_code)]

        #derive_tokens
        pub enum #enum_name {
            #(#keys,)*
        }
    };

    if let Some(const_name) = const_name {
        tokens = quote! {
            #tokens

            impl #enum_name {
                pub const #const_name: &'static [#enum_name] = &[#(#enum_name::#const_keys,)*];
            }
        };
    }

    if let Some(first_variant) = first_variant {
        if options.first_variant_is_default {
            tokens = quote! {
                #tokens

                impl Default for #enum_name {
                    fn default() -> Self {
                        Self::#first_variant
                    }
                }
            };
        }
    }

    if options.impl_display {
        tokens = quote! {
            #tokens

            impl std::fmt::Display for #enum_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
        };
    }

    if options.impl_from_str && options.all_variants_const.is_some() {
        tokens = quote! {
            #tokens

            impl std::str::FromStr for #enum_name {
                type Err = ();

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    const STRINGS: &'static [&'static str] = &[
                        #(#string_keys,)*
                    ];

                    for (index, &key) in STRINGS.iter().enumerate() {
                        if key == s {
                            return Ok(#enum_name::#from_str_const_name[index]);
                        }
                    }

                    Err(())
                }
            }
        };
    }

    tokens.to_string()
}

fn generate_struct_declarations(
    output: &mut String,
    struct_value: &GenericStruct,
    options: &StructOptions,
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
        let mut derived_traits = options.derived_traits.clone();
        if let Some((ser, de)) = options.serde_support.should_derive_ser_de() {
            let prefix = if options.use_serde_derive_crate {
                "serde_derive::"
            } else {
                "serde::"
            };

            if ser {
                derived_traits.push(format!("{}Serialize", prefix));
            }
            if de {
                derived_traits.push(format!("{}Deserialize", prefix));
            }
        }

        if derived_traits.is_empty() {
            "".to_owned()
        } else {
            format!("#[derive({})]\n", derived_traits.join(", "))
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
