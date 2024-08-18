mod util_tester;

use crate::get_paths;
use crate::import;

#[test]
fn test_get_paths() {
    let paths = get_paths();
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
    let paths = get_paths();
    let result = import::import_files(paths);
    assert_eq!(result, Ok());
}
