# Tests

First of all, thanks for your intention on contributing to this project.

In this crate we aim for stability and ease of use for all the macros the crate
declares, we want to be helpful not a burden. To accomplish this we need to test
every part of the crate.

There are two types of test declared in this crate.

## Integration Testing

Integration testing helps us test the user experience, what errors should the user
receive on miss-use of a macro whether it's runtime or not.

The integration tests that pass should be prefixed as `pass_`, otherwise as `fail_`,
the structure for the tests is separated by parameters, so `language/` parameter,
`path/` parameter and `templates/` parameters. Environments is meant to simulate
miss-configuration and the respective errors that should give.

The tests that pass should also be tested in runtime, so added to the mod file as
modules and annotated conditionally with `#[cfg(test)] #[test]`.

## Unitary Testing

Unitary testing is simpler, as it's only functions possessing functions usually from
`translatable::shared`, each module should have its own file and every function
in the module should be tested.

## Running the tests

This project uses make for some command recipes. You can run `make test` and it will
test the application with the correct parameters.

If you are using `cargo test` directly make sure to run the tests with `--test-threds=1`,
there are locks in place so nothing happens, but to make sure you should do that anyway.
