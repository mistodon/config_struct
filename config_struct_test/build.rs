extern crate config_struct;


fn main()
{
    use config_struct::toml_parsing;

    toml_parsing::create_module_from_config("config.toml", "src/config/toml.rs");
}
