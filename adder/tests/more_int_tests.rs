mod common;

// trivial test that demonstrates each test crate can submodule the same code
#[test]
fn everyone_can_submodule() {
    common::setup();
    ()
}
