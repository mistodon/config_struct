#![cfg(test)]

#[macro_use]
extern crate serde_derive;
extern crate ron;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;

mod config;

mod json_tests {
    use serde_json;
    use std;

    use config::json::{Config, CONFIG};

    #[test]
    fn test_declarations() {
        let _conf: &Config = &CONFIG;
    }

    #[test]
    fn test_deserialization() {
        let json_source = include_str!("../config.json");
        let conf: Config = serde_json::from_str(json_source).unwrap();
        assert_eq!(conf.name, "Config name");
    }

    #[test]
    fn test_load_function() {
        let config = Config::load();
        assert_eq!(config.name, CONFIG.name);
    }

    #[test]
    fn test_simple_values() {
        assert_eq!(CONFIG.name, "Config name");
        assert_eq!(CONFIG.nothing, None);
        assert_eq!(CONFIG.number, 100);
        assert_eq!(CONFIG.is_config, true);
        assert_eq!(CONFIG.is_not_config, false);
        assert_eq!(CONFIG.i64_max, std::i64::MAX);
        assert_eq!(CONFIG.u64_max, std::u64::MAX);
        assert_eq!(CONFIG.floaty, 123.456789);
    }

    #[test]
    fn test_composite_values() {
        assert_eq!(CONFIG.coord, [-5.0, 5.0].as_ref());
        assert_eq!(CONFIG.nested.name, "nested2");
        assert_eq!(CONFIG.nested.values.x, 0);
        assert_eq!(CONFIG.nested.values.y, 1);
        assert_eq!(CONFIG.nested.values.z, 2);
        assert_eq!(CONFIG.array_of_structs[0].name, "first");
        assert_eq!(CONFIG.array_of_structs[1].name, "second");
        assert_eq!(CONFIG.array_of_structs[0].n, 0);
        assert_eq!(CONFIG.array_of_structs[1].n, 1);
    }

    #[test]
    fn test_empty_array_is_array_of_unit() {
        let empty: &[()] = &[];
        assert_eq!(CONFIG.empty, empty);
    }
}

mod ron_tests {
    use ron;

    use config::ron::{RonConfig, RONCONFIG};

    #[test]
    fn test_declarations() {
        let _conf: &RonConfig = &RONCONFIG;
    }

    #[test]
    fn test_deserialization() {
        let ron_source = include_str!("../config.ron");
        let conf: RonConfig = ron::de::from_str(ron_source).unwrap();
        assert_eq!(conf.name, "Config name");
    }

    #[test]
    fn test_load_function() {
        let config = RonConfig::load();
        assert_eq!(config.name, RONCONFIG.name);
    }

    #[test]
    fn test_simple_values() {
        assert_eq!(RONCONFIG.name, "Config name");
        assert_eq!(RONCONFIG.unit, ());
        assert_eq!(RONCONFIG.angelface, 'A');
        assert_eq!(RONCONFIG.integer, 100);
        assert_eq!(RONCONFIG.float, 100.1);
        assert_eq!(RONCONFIG.is_true, true);
        assert_eq!(RONCONFIG.nothing, None);
        assert_eq!(RONCONFIG.something, Some(10));
    }

    #[test]
    fn test_empty_array() {
        let empty: &[()] = &[];
        assert_eq!(RONCONFIG.empty, empty);
    }

    #[test]
    fn test_compound_values() {
        assert_eq!(RONCONFIG.countdown[0], 3);
        assert_eq!(RONCONFIG.countdown[1], 2);
        assert_eq!(RONCONFIG.countdown[2], 1);
        assert_eq!(RONCONFIG.structure.name, "Doesn't have one, sadly.");
        assert_eq!(RONCONFIG.structure.status, "Naw too bad.");
        assert_eq!(RONCONFIG.objects[0].name, "Thing 1");
        assert_eq!(RONCONFIG.objects[0].index, 0);
        assert_eq!(RONCONFIG.objects[1].name, "Thing 2");
        assert_eq!(RONCONFIG.objects[1].index, 1);
    }
}

mod toml_tests {
    use toml;

    use config::toml::{TomlConfig, TOMLCONFIG};

    #[test]
    fn test_declarations() {
        let _conf: &TomlConfig = &TOMLCONFIG;
    }

    #[test]
    fn test_deserialization() {
        let toml_source = include_str!("../config.toml");
        let conf: TomlConfig = toml::from_str(toml_source).unwrap();
        assert_eq!(conf.name, "Config name");
    }

    #[test]
    fn test_load_function() {
        let config = TomlConfig::load();
        assert_eq!(config.name, TOMLCONFIG.name);
    }

    #[test]
    fn test_simple_values() {
        assert_eq!(TOMLCONFIG.name, "Config name");
        assert_eq!(TOMLCONFIG.number, 100);
        assert_eq!(TOMLCONFIG.is_config, true);
        assert_eq!(TOMLCONFIG.is_not_config, false);
        assert_eq!(TOMLCONFIG.one_point_zero, 1.0);
        assert_eq!(TOMLCONFIG.one_point_five, 1.5);
        assert_eq!(TOMLCONFIG.floaty, 123.456789);
    }

    #[test]
    fn test_simple_array_values() {
        assert_eq!(TOMLCONFIG.coord, [-5.0, 5.0].as_ref());
        assert_eq!(TOMLCONFIG.color, [0, 64, 128, 255].as_ref());
        assert_eq!(TOMLCONFIG.words, ["one", "two", "three"].as_ref());
        assert_eq!(
            TOMLCONFIG.points,
            [[1, 2].as_ref(), [3, 4].as_ref(), [5, 6].as_ref()].as_ref()
        );
    }

    #[test]
    fn test_table_values() {
        assert_eq!(TOMLCONFIG.table.name, "A table");
        assert_eq!(TOMLCONFIG.table.magnitude, 1000000000);
    }

    #[test]
    fn test_nested_tables() {
        assert_eq!(
            TOMLCONFIG.table.table_again.name,
            "OK this is just getting ridiculous"
        );
        assert_eq!(
            TOMLCONFIG.table.table_again.description,
            "getting ridiculous"
        );
    }

    #[test]
    fn test_array_of_tables() {
        assert_eq!(TOMLCONFIG.arrayble[0].description, "just unbelievable");
        assert_eq!(TOMLCONFIG.arrayble[1].description, "what is this syntax");
    }

    #[test]
    fn test_empty_array_is_array_of_unit() {
        let empty: &[()] = &[];
        assert_eq!(TOMLCONFIG.empty, empty);
    }
}

mod yaml_tests {
    use serde_yaml;
    use std;

    use config::yaml::{YamlConfig, YAML_CONFIG};

    #[test]
    fn test_declarations() {
        let _conf: &YamlConfig = &YAML_CONFIG;
    }

    #[test]
    fn test_deserialization() {
        let yaml_source = include_str!("../config.yaml");
        let conf: YamlConfig = serde_yaml::from_str(yaml_source).unwrap();
        assert_eq!(conf.name, "Config name");
    }

    #[test]
    fn test_load_function() {
        let config = YamlConfig::load();
        assert_eq!(config.name, YAML_CONFIG.name);
    }

    #[test]
    fn test_simple_values() {
        assert_eq!(YAML_CONFIG.name, "Config name");
        assert_eq!(YAML_CONFIG.nothing, None);
        assert_eq!(YAML_CONFIG.number, 100);
        assert_eq!(YAML_CONFIG.is_config, true);
        assert_eq!(YAML_CONFIG.is_not_config, false);
        assert_eq!(YAML_CONFIG.i64_max, std::i64::MAX);
        assert_eq!(YAML_CONFIG.u64_max, std::u64::MAX);
        assert_eq!(YAML_CONFIG.floaty, 123.456789);
    }

    #[test]
    fn test_composite_values() {
        assert_eq!(YAML_CONFIG.coord, [-5.0, 5.0].as_ref());
        assert_eq!(YAML_CONFIG.nested.name, "nested2");
        assert_eq!(YAML_CONFIG.nested.values.x, 0);
        assert_eq!(YAML_CONFIG.nested.values.y, 1);
        assert_eq!(YAML_CONFIG.nested.values.z, 2);
        assert_eq!(YAML_CONFIG.array_of_structs[0].name, "first");
        assert_eq!(YAML_CONFIG.array_of_structs[1].name, "second");
        assert_eq!(YAML_CONFIG.array_of_structs[0].n, 0);
        assert_eq!(YAML_CONFIG.array_of_structs[1].n, 1);
    }

    #[test]
    fn test_empty_array_is_array_of_unit() {
        let empty: &[()] = &[];
        assert_eq!(YAML_CONFIG.empty, empty);
    }
}
