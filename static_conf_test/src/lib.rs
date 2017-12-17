#![cfg(test)]

#[macro_use]
extern crate serde_derive;

mod config;

use config::{ Config, CONFIG };


#[test]
fn test_declarations()
{
    let _conf: &Config = &CONFIG;
}

#[test]
fn test_simple_values()
{
    assert_eq!(CONFIG.name, "Config name");
}
