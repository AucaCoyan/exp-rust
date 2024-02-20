use trycmd::TestCases;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .case("tests/cmd/*.toml")
        .insert_var("[VERSION]", VERSION)
        .unwrap()
        .case("tests/expected_result.md")
        // See Issue #314
        .fail("tests/cmd/buggy-case.toml");
}
