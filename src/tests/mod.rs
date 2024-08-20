mod util_tester;
mod import;

#[test]
fn run_test() {
    let result = crate::run(true);
    assert!(result.is_ok());
}
