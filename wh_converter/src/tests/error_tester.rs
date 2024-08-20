#[test]
fn exit_code_enum_test() {
    use crate::error;

    let exit_code = error::Error::Generic(());
    assert_eq!(exit_code as i32, 1);
}
