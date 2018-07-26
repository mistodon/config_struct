Quick notes for development
===

## API changes

// Generates source string
generate_config(filename, options) // Autodetects format from extension
generate_config_with_format(format, filename, options)
generate_config_from_source(format, source, options)

// Generates source string and writes to file
create_config(filename, destination, options) // Autodetects format from extension
create_config_with_format(format, filename, destination, options)
create_config_from_source(format, source, destination, options)

## Options

0.  (property_name) = (default value)
1.  struct_name = "Config"
2.  const_name = struct_name.to_uppercase()
3.  derived_traits = ["Debug", "Clone", "Serialize", "Deserialize"]
4.  default_float_size = f64
5.  default_int_size = i64
6.  generate_const = true
7.  generate_load_fns = true
8.  runtime_config_path = None (Autodetect from filename arg)
9.  load_dynamically = LoadDynamically::DebugOnly*
10. max_array_size = 0 // Meaning all "arrays" are slices/vecs

*LoadDynamically = {Always, DebugOnly, Never}
    (This overrides generate_const if it not set to Always.)

## Outline

1.  create/generate_config(...)
2.  config = match format {
        [cfg(feature = json)]
        Json => json_parsing::parse(..),
        ...
    }
3.  output = generate_struct_code(config)
        + maybe_generate_const_code(config)
        + maybe_generate_impls(config);
4.  Return or write output
