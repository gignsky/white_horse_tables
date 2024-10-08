use crate::utils;
use crate::import;
use crate::config;

#[test]
fn file_paths_struct_test() {
    let paths = utils::FilePaths {
        square: Some("path/to/square".to_string()),
        tickera: Some("path/to/tickera".to_string()),
        woo_commerce: Some("path/to/woo_commerce".to_string()),
        output: Some("path/to/output".to_string()),
    };

    assert_eq!(paths.square, Some("path/to/square".to_string()));
    assert_eq!(paths.tickera, Some("path/to/tickera".to_string()));
    assert_eq!(paths.woo_commerce, Some("path/to/woo_commerce".to_string()));
    assert_eq!(paths.output, Some("path/to/output".to_string()));
}

#[test]
fn test_get_source_paths() {
    let paths = utils::get_source_paths(config::BASE_PATH, config::TEST_MODE);
    assert!(paths.square.is_some());
    assert!(paths.tickera.is_some());
    assert!(paths.woo_commerce.is_some());
    assert_eq!(paths.square.unwrap(), "/home/gig/local_repos/white_horse_tables/resources/small-copies/Square - items-2024-07-15-2024-08-09.csv");
    assert_eq!(paths.tickera.unwrap(), "/home/gig/local_repos/white_horse_tables/resources/small-copies/Tickera - Attendee List (24).csv");
    assert_eq!(paths.woo_commerce.unwrap(), "/home/gig/local_repos/white_horse_tables/resources/small-copies/WooCommerce - wc-orders-report-export-17232178944415.csv");
    assert!(paths.output.is_none());
}

#[test]
fn test_import_files_with_valid_paths() {
    let paths = utils::get_source_paths(config::BASE_PATH, config::TEST_MODE);
    let result = import::import_files(paths);
    // assert_eq!(result, Ok());
}

#[test]
fn exit_code_enum_test() {
    let exit_code = utils::ExitCode::GeneralError;
    assert_eq!(exit_code as i32, 1);
}
