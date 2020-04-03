Development
===

## Soon:

- [x] Remove `_with_format` variants of functions in lieu of an `Option<Format>` field in the Options structs.
- [x] Document enum generation
- [x] Document enum generation at the top level, and in README
- [x] Document that leaving out the const name will not generate a const
- [x] Add options to implement Display and FromStr
- [x] Add tests for enum generation
- [x] Go over docs and README
- [x] Release
- [/] Add feature-gated generation of an enum from filenames in a directory
    - Should be (option for) repr(<int-type>)
    - Should be guaranteed to be alphabetical
    - And ignore subdirectories
    - Should have a const method to return the original filename
    - Should ignore the full parent path - since that's known at generation time
- [ ] Release again
- [ ] Address TODOs

## Maybe:
- [ ] Consider replacing struct generation with use of the `quote!` crate


## Eventually:
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
    -   e.g. `{"a__f32": 1.0, "b__f64": 1.0}`
6.  And also possibly custom structs to be imported
    -   e.g. `{ "a__MyStruct": {...}}` => `use MyStruct; ...` etc.
