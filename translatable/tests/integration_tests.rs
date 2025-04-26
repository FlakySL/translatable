use std::env::set_var;
use std::env::var;
use std::fs::canonicalize;
use trybuild::TestCases;

// so dynamic tests also run.
#[allow(unused_imports)]
use integration::language::*;
#[allow(unused_imports)]
use integration::path::*;
#[allow(unused_imports)]
use integration::templates::*;

mod integration;

const PATH_ENV: &str = "TRANSLATABLE_LOCALES_PATH";
const OVERLAP_ENV: &str = "TRANSLATABLE_OVERLAP";
const SEEK_MODE_ENV: &str = "TRANSLATABLE_SEEK_MODE";

fn set_locales_env(env: &str) {
    unsafe {
        set_var(
            PATH_ENV,
            canonicalize(format!("./tests/environments/{env}/translations/"))
                .unwrap()
        );
    }
}

#[test]
fn compile_tests() {
    let t = TestCases::new();

    set_locales_env("everything_valid");

    t.pass("./tests/integration/language/pass*.rs");
    t.compile_fail("./tests/integration/language/fail*.rs");

    t.pass("./tests/integration/path/pass*.rs");
    t.compile_fail("./tests/integration/path/fail*.rs");

    t.pass("./tests/integration/templates/pass*.rs");
    t.compile_fail("./tests/integration/templates/fail*.rs");

    // invalid path in configuration.
//    t.compile_fail("../../integration/config/fail_config_path_missmatch.rs");

    // invalid enum value in configuration.
//    t.compile_fail("../../integration/config/fail_config_invalid_enums.rs");

    // translation file rule broken.
//    t.compile_fail("../../integration/config/fail_translations_malformed.rs");
}
