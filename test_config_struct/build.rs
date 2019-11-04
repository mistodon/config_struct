fn main() {
    use config_struct::{DynamicLoading, EnumOptions, StructOptions};

    std::fs::create_dir_all("src/config").expect("Failed to create config dir.");

    println!("cargo:rerun-if-changed=config.json");
    println!("cargo:rerun-if-changed=config.ron");
    println!("cargo:rerun-if-changed=config.toml");
    println!("cargo:rerun-if-changed=config.yaml");
    println!("cargo:rerun-if-changed=enum.json");
    println!("cargo:rerun-if-changed=enum.ron");
    println!("cargo:rerun-if-changed=enum.toml");
    println!("cargo:rerun-if-changed=enum.yaml");
    println!("cargo:rerun-if-changed=tests/atlernate_config.json");
    println!("cargo:rerun-if-changed=tests/example_config.json");

    std::fs::create_dir_all("tests/temp").expect("Failed to create temp test dir.");
    std::fs::copy(
        "tests/alternate_config.json",
        "tests/temp/alternate_config.json",
    )
    .expect("Failed to copy alternate_config.json to temp");
    std::fs::copy(
        "tests/example_config.json",
        "tests/temp/example_config.json",
    )
    .expect("Failed to copy example_config.json to temp");

    config_struct::create_struct(
        "config.json",
        "src/config/json.rs",
        &StructOptions::serde_default(),
    )
    .unwrap();

    config_struct::create_struct(
        "config.ron",
        "src/config/ron.rs",
        &StructOptions {
            struct_name: "RonConfig".to_owned(),
            ..StructOptions::serde_default()
        },
    )
    .unwrap();

    config_struct::create_struct(
        "config.toml",
        "src/config/toml.rs",
        &StructOptions {
            struct_name: "TomlConfig".to_owned(),
            ..StructOptions::serde_default()
        },
    )
    .unwrap();

    config_struct::create_struct(
        "config.yaml",
        "src/config/yaml.rs",
        &StructOptions {
            struct_name: "YamlConfig".to_owned(),
            const_name: Some("YAML_CONFIG".to_owned()),
            ..StructOptions::serde_default()
        },
    )
    .unwrap();

    config_struct::create_struct(
        "tests/temp/example_config.json",
        "tests/config/dynamic.rs",
        &StructOptions {
            struct_name: "DynamicConfig".to_owned(),
            const_name: Some("DYNAMIC_CONFIG".to_owned()),
            dynamic_loading: DynamicLoading::Always,
            ..StructOptions::serde_default()
        },
    )
    .unwrap();

    config_struct::create_struct(
        "tests/temp/example_config.json",
        "tests/config/dependent.rs",
        &StructOptions {
            struct_name: "DependentConfig".to_owned(),
            const_name: Some("DEPENDENT_CONFIG".to_owned()),
            dynamic_loading: DynamicLoading::DebugOnly,
            ..StructOptions::serde_default()
        },
    )
    .unwrap();

    config_struct::create_struct(
        "tests/temp/example_config.json",
        "tests/config/static_config.rs",
        &StructOptions {
            struct_name: "StaticConfig".to_owned(),
            const_name: Some("STATIC_CONFIG".to_owned()),
            dynamic_loading: DynamicLoading::Never,
            ..StructOptions::serde_default()
        },
    )
    .unwrap();

    config_struct::create_enum(
        "enum.json",
        "src/config/json_enum.rs",
        &EnumOptions::serde_default(),
    )
    .unwrap();

    config_struct::create_enum(
        "enum.ron",
        "src/config/ron_enum.rs",
        &EnumOptions {
            enum_name: "RonEnum".to_owned(),
            ..EnumOptions::serde_default()
        },
    )
    .unwrap();

    config_struct::create_enum(
        "enum.toml",
        "src/config/toml_enum.rs",
        &EnumOptions {
            enum_name: "TomlEnum".to_owned(),
            ..EnumOptions::serde_default()
        },
    )
    .unwrap();

    config_struct::create_enum(
        "enum.yaml",
        "src/config/yaml_enum.rs",
        &EnumOptions {
            enum_name: "YamlEnum".to_owned(),
            all_variants_const: Some("VALUES".to_owned()),
            ..EnumOptions::serde_default()
        },
    )
    .unwrap();
}
