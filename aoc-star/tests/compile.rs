#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/ui/star_ok.rs");
    t.compile_fail("tests/ui/star_invalid_args.rs");
}
