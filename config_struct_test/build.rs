extern crate config_struct;


fn main()
{
    use config_struct::toml_parsing;
    use config_struct::yaml_parsing;

    let toml_config = toml_parsing::parse_config_from_file("config.toml").unwrap();
    config_struct::write_config_module(&toml_config, "src/config/toml.rs").unwrap();

    let yaml_config = yaml_parsing::parse_config_from_file("config.yaml").unwrap();
    config_struct::write_config_module(&yaml_config, "src/config/yaml.rs").unwrap();
}
