use std::env::{remove_var, set_var};
use std::fs::canonicalize;
use std::sync::Mutex;

use trybuild::TestCases;

mod integration;

const PATH_ENV: &str = "TRANSLATABLE_LOCALES_PATH";
const OVERLAP_ENV: &str = "TRANSLATABLE_OVERLAP";

static ENV_MUTEX: Mutex<()> = Mutex::new(());

macro_rules! lock_env {
    () => {
        let _env_guard = ENV_MUTEX.lock();
    };
}

#[inline]
unsafe fn set_default_env() {
    unsafe {
        set_locales_env("everything_valid");
        remove_var(OVERLAP_ENV);
    }
}

#[inline]
unsafe fn set_locales_env(env: &str) {
    unsafe {
        set_var(
            PATH_ENV,
            canonicalize(format!("./tests/environments/{env}/translations/")).unwrap(),
        );
    }
}

#[test]
fn valid_environment() {
    unsafe {
        let t = TestCases::new();

        lock_env!();

        set_default_env();
        set_locales_env("everything_valid");

        t.pass("./tests/integration/translation/language/pass*.rs");
        t.compile_fail("./tests/integration/translation/language/fail*.rs");

        t.pass("./tests/integration/translation/path/pass*.rs");
        t.compile_fail("./tests/integration/translation/path/fail*.rs");

        t.pass("./tests/integration/translation/templates/pass*.rs");
        t.compile_fail("./tests/integration/translation/templates/fail*.rs");

        t.pass("./tests/integration/context/pass*.rs");
        t.compile_fail("./tests/integration/context/fail*.rs");
    }
}

#[test]
fn invalid_tests_path() {
    unsafe {
        let t = TestCases::new();

        lock_env!();

        set_default_env();
        set_var(PATH_ENV, "something_invalid");

        // invalid path in configuration.
        t.compile_fail("./tests/integration/config/fail_config_path_missmatch.rs");
    }
}

#[test]
fn invalid_config_value() {
    unsafe {
        let t = TestCases::new();

        lock_env!();

        set_default_env();
        set_locales_env("everything_valid");
        set_var(OVERLAP_ENV, "49854835093459fjkdjfkj");

        // invalid enum value in configuration.
        t.compile_fail("./tests/integration/config/fail_config_invalid_enums.rs");
    }
}

#[test]
fn translations_malformed() {
    unsafe {
        let t = TestCases::new();

        lock_env!();

        set_default_env();
        set_locales_env("translations_malformed");

        // translation file rule broken.
        t.compile_fail("./tests/integration/config/fail_translations_malformed.rs");
    }
}
