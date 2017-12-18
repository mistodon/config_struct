extern crate config_struct;


fn main()
{
    use config_struct::toml_parsing;
    use config_struct::yaml_parsing;

    toml_parsing::create_module_from_config("config.toml", "src/config/toml.rs");
    yaml_parsing::create_module_from_config("config.yaml", "src/config/yaml.rs");
}
