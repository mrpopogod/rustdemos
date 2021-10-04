// important note: you need a lib.rs file (which creates a library crate) to integration test.
// a binary-only create (main.rs and no lib.rs) won't be able to integration test because
// a binary crate isn't expected to expose functions, so there would be nothing for the
// tests to glom onto

use adder;

mod common; // if we want to have common modules, they need to be identified as submodules

#[test]
fn it_greets_by_name() {
    common::setup();
    let result = adder::greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
