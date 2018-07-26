extern crate config_struct;

fn main() {
    use config_struct::{json_parsing, ron_parsing, toml_parsing, yaml_parsing, Options};

    let json_config = json_parsing::parse_config_from_file("config.json").unwrap();
    config_struct::write_config_module("src/config/json.rs", &json_config, &Options::default())
        .unwrap();

    let ron_options = &Options {
        struct_name: "RonConfig".to_owned(),
        ..Default::default()
    };
    let ron_config = ron_parsing::parse_config_from_file("config.ron").unwrap();
    config_struct::write_config_module("src/config/ron.rs", &ron_config, ron_options).unwrap();

    let toml_options = &Options {
        struct_name: "TomlConfig".to_owned(),
        ..Default::default()
    };
    let toml_config = toml_parsing::parse_config_from_file("config.toml").unwrap();
    config_struct::write_config_module("src/config/toml.rs", &toml_config, toml_options).unwrap();

    let yaml_options = &Options {
        struct_name: "YamlConfig".to_owned(),
        const_name: Some("YAML_CONFIG".to_owned()),
        ..Default::default()
    };
    let yaml_config = yaml_parsing::parse_config_from_file("config.yaml").unwrap();
    config_struct::write_config_module("src/config/yaml.rs", &yaml_config, yaml_options).unwrap();
}
