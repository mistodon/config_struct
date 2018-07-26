use std::collections::BTreeMap;
use std::io;
use std::path::Path;

use value::{RawStructValue, RawValue};

pub type ParsedFields<T> = BTreeMap<String, T>;

pub fn parsed_to_raw_config<T, F>(parsed_config: ParsedFields<T>, convert_fn: F) -> RawStructValue
where
    F: Fn(&str, &str, T) -> RawValue,
{
    let struct_name = "Config".to_owned();

    let fields = parsed_config
        .into_iter()
        .map(|(key, value)| {
            let value = convert_fn("_Config", &key, value);
            (key, value)
        })
        .collect();

    RawStructValue {
        struct_name,
        fields,
    }
}

pub fn slurp_file(path: &Path) -> Result<String, io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut buffer = String::new();
    let file = &mut File::open(&path)?;
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}
