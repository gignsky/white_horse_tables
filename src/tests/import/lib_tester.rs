use crate::import::lib::{MasterImportedTables, RowType, Table, Source};

#[test]
fn master_imported_tables_struct_test() {
    let tables = MasterImportedTables {
        square: Ok(Table {
            row_type: RowType::InitalRow,
            table_source: Source::Square,
            rows: Vec::new(),
        }),
    };

    assert!(tables.square.is_ok());
}

#[test]
fn row_type_enum_test() {
    let row_type = RowType::InitalRow;
    assert_eq!(row_type, RowType::InitalRow);
    let row_type = RowType::FinalRow;
    assert_eq!(row_type, RowType::FinalRow);
}

#[test]
fn source_enum_test() {
	for source in vec![Source::Square, Source::Tickera, Source::WooCommerce] {
		match source {
			Source::Square => assert_eq!(source, Source::Square),
			Source::Tickera => assert_eq!(source, Source::Tickera),
			Source::WooCommerce => assert_eq!(source, Source::WooCommerce),
		}
	}
}
