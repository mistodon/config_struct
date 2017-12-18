extern crate config_struct;


fn main()
{
    config_struct::construct_config("config.toml", "src/config.rs");
}
