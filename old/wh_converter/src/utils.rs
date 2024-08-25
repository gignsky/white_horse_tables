//! Utility functions and types used by the program.

/// Imports the `tinyfiledialogs` crate as `tfd`.
///
/// This allows you to use the functions and types provided by the `tinyfiledialogs` crate
/// by referring to them through the `tfd` namespace.
///
/// Example usage:
/// ```
/// use tinyfiledialogs as tfd;
/// ```
///
/// For more information, refer to the documentation of the `tinyfiledialogs` crate.
use tinyfiledialogs as tfd;

/// Imports the `FilePaths` struct from the `containers` module.
use super::lib::containers::FilePaths;

/// Returns the file paths of the csv files to be imported.
///
/// The function will return the file paths of the csv files to be imported.
/// If the test_mode is set to true, the function will return the file paths of the test csv files.
/// If the test_mode is set to false, the function will open a file dialog box to request the file paths from the user.
pub fn get_source_paths(base_path: &str, test_mode: bool) -> FilePaths {
    let (square, tickera, woo_commerce) = if test_mode {
        (
            "Square - items-2024-07-15-2024-08-09.csv".to_string(),
            "Tickera - Attendee List (24).csv".to_string(),
            "WooCommerce - wc-orders-report-export-17232178944415.csv".to_string(),
        )
    } else {
        (
            request_path("Square", base_path.to_string(), None),
            request_path("Tickera", base_path.to_string(), None),
            request_path("WooCommerce", base_path.to_string(), None),
        )
    };
    let square_file_path = Some(if test_mode { format!("{}{}", base_path, square) } else { square });
    let tickera_file_path = Some(if test_mode { format!("{}{}", base_path, tickera) } else { tickera });
    let woo_commerce_file_path = Some(if test_mode {
        format!("{}{}", base_path, woo_commerce)
    } else {
        woo_commerce
    });

    FilePaths {
        square: square_file_path,
        tickera: tickera_file_path,
        woo_commerce: woo_commerce_file_path,
        output: None,
    }
}

/// Requests a file path from the user.
///
/// unless the test_mode is set to true, the function will open a file dialog box to request a file path from the user.
/// in the case of test_mode being set to true, the function will return a default path of "path/to/file"
fn request_path(title: &str, default_path: String, test_mode: Option<bool>) -> String {
    let path = match test_mode {
        Some(true) => Some("path/to/file".to_string()),
        _ => tfd::open_file_dialog(title, &default_path, None)
    };

    match path {
        Some(path) => {
            println!("Importing from path: {:?}", path);
            path
        }
        None => panic!("No file path provided"), //TODO - add a dialog box here, for reselection of file path, maybe use this as an opportunity to throw a custom error with a result.
    }
}

#[test]
fn request_path_test() {
    let path = request_path("Test", "path/to/file".to_string(), Some(true));
    assert_eq!(path, "path/to/file".to_string());
}
