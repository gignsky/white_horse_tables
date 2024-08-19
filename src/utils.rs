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

/// Represents a collection of imported csv files in table format.
pub struct MasterImportedTables {
	/// The imported square table
	pub square: Result<String, csv::Error>,
	// /// The imported tickera table
	// pub tickera: String,
	// /// The imported WooCommerce table
	// pub woo_commerce: String,
}

// /// Possible sources of the imported tables
// #[derive(PartialEq, Eq, Debug)]
// enum Source {
// 	Square,
// 	Tickera,
// 	WooCommerce,
// }
//
// #[test]
// fn source_enum_test() {
// 	for source in vec![Source::Square, Source::Tickera, Source::WooCommerce] {
// 		match source {
// 			Source::Square => assert_eq!(source, Source::Square),
// 			Source::Tickera => assert_eq!(source, Source::Tickera),
// 			Source::WooCommerce => assert_eq!(source, Source::WooCommerce),
// 		}
// 	}
// }

//
// pub struct ImportedTable {
//     pub source: Source,
//     pub table: [Vec<u64>, Vec<T>]
// }

/// Represents the file paths of the imported csv files and the output file path.
#[derive(Debug)]
pub struct FilePaths {
	///Path to square csv file
	pub square: Option<String>,
	///Path to tickera csv file
	pub tickera: Option<String>,
	///Path to WooCommerce csv file
	pub woo_commerce: Option<String>,
	///Path to output csv file folder
	pub output: Option<String>,
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

/// Returns the file paths of the csv files to be imported.
///
/// The function will return the file paths of the csv files to be imported.
/// If the test_mode is set to true, the function will return the file paths of the test csv files.
/// If the test_mode is set to false, the function will open a file dialog box to request the file paths from the user.
pub fn get_source_paths(test_mode: Option<bool>) -> FilePaths {
	let base_path = "/home/gig/local_repos/white_horse_tables/resources/small-copies";
	let (square, tickera, woo_commerce) = if test_mode == Some(true) {
		(
			"Square - items-2024-07-15-2024-08-09.csv".to_string(),
			"Tickera - Attendee List (24)".to_string(),
			"WooCommerce - wc-orders-report-export-17232178944415.csv".to_string(),
		)
	} else {
		(
			request_path("Square", base_path.to_string(), None),
			request_path("Tickera", base_path.to_string(), None),
			request_path("WooCommerce", base_path.to_string(), None),
		)
	};
	let square_file_path = Some(if test_mode == Some(true) { format!("{}/{}", base_path, square) } else { square });
	let tickera_file_path = Some(if test_mode == Some(true) { format!("{}/{}", base_path, tickera) } else { tickera });
	let woo_commerce_file_path = Some(if test_mode == Some(true) {
		format!("{}/{}", base_path, woo_commerce)
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

/// Controls what exit code to give when the program exits.
///
/// this depends on what error causes the program to exit.
pub enum ExitCode {
    /// The program exited with a general error that has not been specifically caught. This indicates a bug in the program.
	GeneralError = 1,
}

/// Exits the program with the given exit code.
///
/// The exit code is a number that indicates the reason for the program's exit.
pub fn exit(code: ExitCode) {
    std::process::exit(code as i32);
}
