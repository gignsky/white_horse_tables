#[test]
fn exit_code_enum_test() {
    use crate::exits;

    let exit_code = exits::ExitCode::GeneralError;
    assert_eq!(exit_code as i32, 1);
}
