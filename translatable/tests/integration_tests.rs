use std::env::set_current_dir;
use trybuild::TestCases;

// so dynamic tests also run.
#[allow(unused_imports)]
use integration::language::*;
#[allow(unused_imports)]
use integration::path::*;
#[allow(unused_imports)]
use integration::templates::*;

mod integration;

fn set_test_environment(environment: &str) {
    set_current_dir(format!("tests/ui/environments/{environment}"))
        .expect("Should be able to change environment.");
}

#[test]
fn compile_tests() {
    let t = TestCases::new();

    // general test cases.
    set_test_environment("everything_valid");

    t.pass("./ui/language/pass*.rs");
    t.compile_fail("./ui/language/fail*.rs");

    t.pass("./ui/path/pass*.rs");
    t.compile_fail("./ui/path/fail*.rs");

    t.pass("./ui/templates/pass*.rs");
    t.compile_fail("./ui/templates/fail*.rs");

    // invalid path in configuration.
    set_test_environment("config_path_missmatch");
    t.compile_fail("./ui/config/fail_config_path_missmatch.rs");

    // invalid enum value in configuration.
    set_test_environment("config_invalid_value");
    t.compile_fail("./ui/config/fail_config_invalid_enums.rs");

    // translation file rule broken.
    set_test_environment("translations_malformed");
    t.compile_fail("./ui/config/fail_translations_malformed.rs");
}
