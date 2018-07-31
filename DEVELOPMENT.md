Development
===

## Need to:
1.  `Options.force_numbers_to_float` (unless it's not required?)
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
