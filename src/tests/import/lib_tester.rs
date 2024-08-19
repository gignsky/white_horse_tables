use crate::import::lib::MasterImportedTables;

#[test]
fn master_imported_tables_struct_test() {
    let tables = utils::MasterImportedTables {
        square: Ok("Square Table".to_string()),
    };

    assert!(tables.square.is_ok());
}
