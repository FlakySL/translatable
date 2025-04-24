use std::env::set_current_dir;

use trybuild::TestCases;

fn set_test_environment(environment: &str) {
    set_current_dir(format!("tests/ui/environments/{environment}"))
        .expect("Should be able to change environment.");
}

#[test]
fn ui_tests() {
    set_test_environment("everything_valid");

    let t = TestCases::new();

    t.pass("./ui/language/pass*.rs");
    t.compile_fail("./ui/language/fail*.rs");

    t.pass("./ui/path/pass*.rs");
    t.compile_fail("./ui/path/fail*.rs");

    t.pass("./ui/templates/pass*.rs");
    t.compile_fail("./ui/templates/fail*.rs");

    // TODO: run each test with it's set environment.
}
