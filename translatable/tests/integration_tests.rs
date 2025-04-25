use std::env::current_dir;
use std::env::set_current_dir;
use std::env::set_var;
use std::sync::OnceLock;
use trybuild::TestCases;

// so dynamic tests also run.
#[allow(unused_imports)]
use integration::language::*;
#[allow(unused_imports)]
use integration::path::*;
#[allow(unused_imports)]
use integration::templates::*;

mod integration;

#[test]
fn compile_tests() {
    let t = TestCases::new();

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
