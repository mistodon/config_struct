extern crate config_struct;

fn main() {
    use config_struct::{self, DynamicLoading, Options};

    let debug_string = &format!("dir: {}", std::env::current_dir().unwrap().display());
    config_struct::create_config("config.json", "src/config/json.rs", &Options::default()).expect(debug_string);

    config_struct::create_config(
        "config.ron",
        "src/config/ron.rs",
        &Options {
            struct_name: "RonConfig".to_owned(),
            ..Default::default()
        },
    ).unwrap();

    config_struct::create_config(
        "config.toml",
        "src/config/toml.rs",
        &Options {
            struct_name: "TomlConfig".to_owned(),
            ..Default::default()
        },
    ).unwrap();

    config_struct::create_config(
        "config.yaml",
        "src/config/yaml.rs",
        &Options {
            struct_name: "YamlConfig".to_owned(),
            const_name: Some("YAML_CONFIG".to_owned()),
            ..Default::default()
        },
    ).unwrap();

    config_struct::create_config(
        "tests/example_config.json",
        "tests/config/dynamic.rs",
        &Options {
            struct_name: "DynamicConfig".to_owned(),
            const_name: Some("DYNAMIC_CONFIG".to_owned()),
            dynamic_loading: DynamicLoading::Always,
            ..Default::default()
        },
    ).unwrap();

    config_struct::create_config(
        "tests/example_config.json",
        "tests/config/dependent.rs",
        &Options {
            struct_name: "DependentConfig".to_owned(),
            const_name: Some("DEPENDENT_CONFIG".to_owned()),
            dynamic_loading: DynamicLoading::DebugOnly,
            ..Default::default()
        },
    ).unwrap();

    config_struct::create_config(
        "tests/example_config.json",
        "tests/config/static_config.rs",
        &Options {
            struct_name: "StaticConfig".to_owned(),
            const_name: Some("STATIC_CONFIG".to_owned()),
            dynamic_loading: DynamicLoading::Never,
            ..Default::default()
        },
    ).unwrap();
}
