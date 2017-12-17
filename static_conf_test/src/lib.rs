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
    assert_eq!(CONFIG.number, 100);
    assert_eq!(CONFIG.is_config, true);
    assert_eq!(CONFIG.is_not_config, false);
    assert_eq!(CONFIG.one_point_zero, 1.0);
    assert_eq!(CONFIG.one_point_five, 1.5);
    assert_eq!(CONFIG.floaty, 123.456789);
}

#[test]
fn test_simple_array_values()
{
    assert_eq!(CONFIG.coord, [-5.0, 5.0].as_ref());
    assert_eq!(CONFIG.color, [0, 64, 128, 255].as_ref());
    assert_eq!(CONFIG.words, ["one", "two", "three"].as_ref());
}
