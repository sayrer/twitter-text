extern crate rustversion;
extern crate trybuild;

#[rustversion::attr(any(before(1.39), not(stable)), ignore)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
