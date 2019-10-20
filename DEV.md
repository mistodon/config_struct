Development
===

## Housekeeping

- [x] Update CI config
    - [x] Ensure it works, obviously
    - [x] With formatting
    - [x] With clippy (not nightly)
    - [x] Make it call ./alltests script to avoid duplication?
- [x] Reintroduce write_only_if_changed option
    - This is to prevent recompilation when config.rs hasn't actually changed. Can't do this with a .gitignore alone.
- [x] Remove need for serde/load_fns in default features/options
    - This is so the default StructOptions compile without any other crates present.
- [x] Have serde_support as an option, so you don't need to specify it in derived traits
    - [x] And test
- [ ] Once-over the docs

## Need to:
1.  `StructOptions.force_numbers_to_float` (unless it's not required?)
2.  Fix issues with RON parsing - documented in that module
3.  Fix up remaining unwraps/expects/unreachables/unimplementeds and return errors
4.  Improve test coverage - particularly unit tests

## Want to:
1.  Try to avoid generating the same structs twice
2.  Move to a more elegant method of code generation (quote! macro?)
3.  Move to a more elegant method of comparing types (not strings!!)
4.  Be smarter about types of collections:
    -   `[1, 2.0]` should resolve to `[f32]`
    -   `[10, null]` should resolve to `[Option<i64>]`
    -   `[[1.0], [1]]` should resolve to `[[f32]]`
    -   `[1.0, "hello"]` should throw a useful error
    -   `[{"a": 1}, {"b": 5.0}]` should resolve to `[struct {a: Option<i64>, b: Option<f64>}]`
5.  Possibly allow some kind of type hints in the config itself
    -   e.g. `{"a_f32": 1.0, "b_f64": 1.0}`
6.  And also possibly custom structs to be imported
    -   e.g. `{ "a_MyStruct": {...}}` => `use MyStruct; ...` etc.
