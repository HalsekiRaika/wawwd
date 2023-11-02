#[test]
fn test() {
    let try_build = trybuild::TestCases::new();
    try_build.pass("tests/ring.rs");
}