#[test]
pub fn examples() {
    let case = trybuild::TestCases::new();
    case.pass("examples/simplest.rs");
}
