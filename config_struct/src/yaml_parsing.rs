use std::path::Path;

use serde_yaml::{ self, Value };

use { RawValue, RawStructValue, create_config_module };


pub fn create_module_from_config<S, D>(config_path: S, module_path: D)
where
    S: AsRef<Path>,
    D: AsRef<Path>
{
    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::{ Read, Write };

    let config_source = {
        let mut buffer = String::new();
        let file = &mut File::open(&config_path).unwrap();
        file.read_to_string(&mut buffer).unwrap();
        buffer
    };

    let yaml_object: BTreeMap<String, Value> = serde_yaml::from_str(&config_source).unwrap();

    let raw_config: RawStructValue = {
        let struct_name = "Config".to_owned();
        let fields = yaml_object.into_iter().map(|(key, value)|
        {
            let value = yaml_to_raw_value("_Config", &key, value);
            (key, value)
        }).collect();
        RawStructValue { struct_name, fields }
    };

    let config_module_source = create_config_module(&raw_config);

    let file = &mut File::create(&module_path).unwrap();
    file.write_all(config_module_source.as_bytes()).unwrap();
}


fn yaml_to_raw_value(super_struct: &str, super_key: &str, value: Value) -> RawValue
{
    match value
    {
        Value::Null => RawValue::Unit,
        Value::Bool(value) => RawValue::Bool(value),
        Value::Number(value) => match (value.as_i64(), value.as_u64(), value.as_f64())
        {
            (Some(x), _, _) => RawValue::I64(x),
            (None, Some(x), _) => RawValue::U64(x),
            (None, None, Some(x)) => RawValue::F64(x),
            _ => unimplemented!("Should handle error here")
        },
        Value::String(value) => RawValue::String(value),
        Value::Sequence(values) => {
            RawValue::Array(values.into_iter()
                .map(|value| yaml_to_raw_value(super_struct, super_key, value))
                .collect())
        },
        Value::Mapping(values) => {
            let sub_struct_name = format!("{}__{}", super_struct, super_key);
            let values = values.into_iter()
                .map(
                    |(key, value)|
                    {
                        let key = key.as_str().unwrap().to_owned();
                        let value = yaml_to_raw_value(&sub_struct_name, &key, value);
                        (key, value)
                    })
                .collect();
            RawValue::Struct(RawStructValue { struct_name: sub_struct_name, fields: values })
        }
    }
}

