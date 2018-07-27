extern crate config_struct;

fn main() {
    use config_struct::{self, Options};

    config_struct::create_config("config.json", "src/config/json.rs", &Options::default()).unwrap();

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
}
