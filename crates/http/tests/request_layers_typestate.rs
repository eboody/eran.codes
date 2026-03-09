#[test]
fn request_layers_typestate_contract() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/request_layers_finish_too_early.rs");
    cases.compile_fail("tests/ui/request_layers_skip_step.rs");
}
