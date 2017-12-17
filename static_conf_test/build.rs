extern crate static_conf;


fn main()
{
    static_conf::construct_config("config.toml", "src/config.rs");
}
