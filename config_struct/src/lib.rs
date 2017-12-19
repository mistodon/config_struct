#[cfg(feature = "ron-parsing")]
extern crate ron;

#[cfg(feature = "toml-parsing")]
extern crate toml;

#[cfg(feature = "yaml-parsing")]
extern crate serde_yaml;


#[cfg(feature = "ron-parsing")]
pub mod ron_parsing;

#[cfg(feature = "toml-parsing")]
pub mod toml_parsing;

#[cfg(feature = "yaml-parsing")]
pub mod yaml_parsing;


extern crate failure;


mod generation;
mod parsing;
mod value;


use std::path::Path;

use failure::Error;

pub use value::{ RawValue, RawStructValue };


pub fn create_config_module(raw_config: &RawStructValue) -> String
{
    let mut code = String::new();

    code.push_str("use std::borrow::Cow;\n\n");

    generation::generate_struct_declarations(&mut code, raw_config);

    code.push_str(
        &format!(
            "pub const CONFIG: Config = {};\n",
            generation::struct_value_string(raw_config, 0)));

    code
}


pub fn write_config_module<P>(raw_config: &RawStructValue, module_path: P) -> Result<(), Error>
where
    P: AsRef<Path>
{
    use std::fs::File;
    use std::io::Write;

    let code = create_config_module(raw_config);

    let file = &mut File::create(module_path)?;
    file.write_all(code.as_bytes())?;

    Ok(())
}

