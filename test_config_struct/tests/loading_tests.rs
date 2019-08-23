#[path = "config/dynamic.rs"]
mod dynamic;

#[path = "config/dependent.rs"]
mod dependent;

#[path = "config/static_config.rs"]
mod static_config;

struct Cleanup;

impl Cleanup {
    fn new() -> Self {
        std::fs::copy(
            "tests/temp/example_config.json",
            "tests/temp/example_config.json.backup",
        )
        .unwrap();
        Cleanup
    }
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        std::fs::rename(
            "tests/temp/example_config.json.backup",
            "tests/temp/example_config.json",
        )
        .unwrap();
    }
}

#[test]
#[cfg(debug_assertions)]
fn test_in_debug_mode() {
    run_loading_tests("Renamed Config", "Alternate Config");
}

#[test]
#[cfg(not(debug_assertions))]
fn test_in_release_mode() {
    run_loading_tests("Example Config", "Example Config");
}

fn run_loading_tests(dependent_renamed: &str, dependent_alternate: &str) {
    let _cleanup = Cleanup::new();

    // Test loading
    let dynamic_conf = dynamic::DynamicConfig::load();
    let dependent_conf = dependent::DependentConfig::load();
    let static_conf = static_config::StaticConfig::load();

    assert_eq!(dynamic_conf.name, "Example Config");
    assert_eq!(dependent_conf.name, "Example Config");
    assert_eq!(static_conf.name, "Example Config");

    std::fs::write(
        "tests/temp/example_config.json",
        br#"{ "name": "Renamed Config" }"#,
    )
    .unwrap();

    // Test reloading with changes
    let dynamic_conf = dynamic::DynamicConfig::load();
    let dependent_conf = dependent::DependentConfig::load();
    let static_conf = static_config::StaticConfig::load();

    assert_eq!(dynamic_conf.name, "Renamed Config");
    assert_eq!(dependent_conf.name, dependent_renamed);
    assert_eq!(static_conf.name, "Example Config");

    // Test loading from a file
    let dynamic_conf =
        dynamic::DynamicConfig::load_from("tests/temp/alternate_config.json".as_ref()).unwrap();
    let dependent_conf =
        dependent::DependentConfig::load_from("tests/temp/alternate_config.json".as_ref()).unwrap();
    let static_conf =
        static_config::StaticConfig::load_from("tests/temp/alternate_config.json".as_ref())
            .unwrap();

    assert_eq!(dynamic_conf.name, "Alternate Config");
    assert_eq!(dependent_conf.name, dependent_alternate);
    assert_eq!(static_conf.name, "Example Config");
}
