#[test]
fn compile_errors() {
    let t = trybuild::TestCases::new();
    t.compile_fail("compile-errors/*.rs");
}
