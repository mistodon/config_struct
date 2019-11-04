#[test]
fn test_number_sizes() {
    use config_struct::{FloatSize, Format, IntSize, StructOptions};

    let basic_options = StructOptions {
        generate_load_fns: false,
        generate_const: false,
        ..Default::default()
    };

    let f32_opts = StructOptions {
        default_float_size: FloatSize::F32,
        ..basic_options.clone()
    };

    let f64_opts = StructOptions {
        default_float_size: FloatSize::F64,
        ..basic_options.clone()
    };

    let i8_opts = StructOptions {
        default_int_size: IntSize::I8,
        ..basic_options.clone()
    };

    let i16_opts = StructOptions {
        default_int_size: IntSize::I16,
        ..basic_options.clone()
    };

    let i32_opts = StructOptions {
        default_int_size: IntSize::I32,
        ..basic_options.clone()
    };

    let i64_opts = StructOptions {
        default_int_size: IntSize::I64,
        ..basic_options.clone()
    };

    let isize_opts = StructOptions {
        default_int_size: IntSize::ISize,
        ..basic_options.clone()
    };

    let float_tests = &[(f32_opts, "f32"), (f64_opts, "f64")];

    let int_tests = &[
        (i8_opts, "i8"),
        (i16_opts, "i16"),
        (i32_opts, "i32"),
        (i64_opts, "i64"),
        (isize_opts, "isize"),
    ];

    let float_inputs = &[
        (Format::Json, r#"{"number": 100.5}"#),
        (Format::Ron, "(number: 100.5)"),
        (Format::Toml, "number = 100.5"),
        (Format::Yaml, "number: 100.5"),
    ];

    let int_inputs = &[
        (Format::Json, r#"{"number": 100}"#),
        (Format::Ron, "(number: 100)"),
        (Format::Toml, "number = 100"),
        (Format::Yaml, "number: 100"),
    ];

    for &(ref options, expected_type) in float_tests {
        for &(format, code) in float_inputs {
            let generated_code =
                config_struct::generate_struct_from_source(format, code, options).unwrap();

            assert!(
                generated_code.contains(expected_type),
                "Expected to find {}",
                expected_type
            );
        }
    }

    for &(ref options, expected_type) in int_tests {
        for &(format, code) in int_inputs {
            let generated_code =
                config_struct::generate_struct_from_source(format, code, options).unwrap();

            assert!(
                generated_code.contains(expected_type),
                "Expected to find {}",
                expected_type
            );
        }
    }
}
