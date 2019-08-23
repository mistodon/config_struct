fn main() {
    use config_struct::{DynamicLoading, SerdeSupport, StructOptions};

    std::fs::create_dir_all("src/config").expect("Failed to create config dir.");

    println!("cargo:rerun-if-changed=config.json");
    println!("cargo:rerun-if-changed=config.ron");
    println!("cargo:rerun-if-changed=config.toml");
    println!("cargo:rerun-if-changed=config.yaml");
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

    config_struct::create_config(
        "config.json",
        "src/config/json.rs",
        &StructOptions {
            serde_support: SerdeSupport::Yes,
            ..Default::default()
        },
    )
    .unwrap();

    config_struct::create_config(
        "config.ron",
        "src/config/ron.rs",
        &StructOptions {
            struct_name: "RonConfig".to_owned(),
            serde_support: SerdeSupport::Yes,
            ..Default::default()
        },
    )
    .unwrap();

    config_struct::create_config(
        "config.toml",
        "src/config/toml.rs",
        &StructOptions {
            struct_name: "TomlConfig".to_owned(),
            serde_support: SerdeSupport::Yes,
            ..Default::default()
        },
    )
    .unwrap();

    config_struct::create_config(
        "config.yaml",
        "src/config/yaml.rs",
        &StructOptions {
            struct_name: "YamlConfig".to_owned(),
            const_name: Some("YAML_CONFIG".to_owned()),
            serde_support: SerdeSupport::Yes,
            ..Default::default()
        },
    )
    .unwrap();

    config_struct::create_config(
        "tests/temp/example_config.json",
        "tests/config/dynamic.rs",
        &StructOptions {
            struct_name: "DynamicConfig".to_owned(),
            const_name: Some("DYNAMIC_CONFIG".to_owned()),
            dynamic_loading: DynamicLoading::Always,
            serde_support: SerdeSupport::Yes,
            ..Default::default()
        },
    )
    .unwrap();

    config_struct::create_config(
        "tests/temp/example_config.json",
        "tests/config/dependent.rs",
        &StructOptions {
            struct_name: "DependentConfig".to_owned(),
            const_name: Some("DEPENDENT_CONFIG".to_owned()),
            dynamic_loading: DynamicLoading::DebugOnly,
            serde_support: SerdeSupport::Yes,
            ..Default::default()
        },
    )
    .unwrap();

    config_struct::create_config(
        "tests/temp/example_config.json",
        "tests/config/static_config.rs",
        &StructOptions {
            struct_name: "StaticConfig".to_owned(),
            const_name: Some("STATIC_CONFIG".to_owned()),
            dynamic_loading: DynamicLoading::Never,
            serde_support: SerdeSupport::Yes,
            ..Default::default()
        },
    )
    .unwrap();
}
