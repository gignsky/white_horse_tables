use tinyfiledialogs as tfd;

pub struct MasterImportedTables {
	pub square: Result<String, csv::Error>, // pub tickera: String,
	                                        // pub woo_commerce: String,
}

enum Source {
	Square,
	Tickera,
	WooCommerce,
}
//
// pub struct ImportedTable {
//     pub source: Source,
//     pub table: [Vec<u64>, Vec<T>]
// }

#[derive(Debug)]
pub struct FilePaths {
	pub square: Option<String>,
	pub tickera: Option<String>,
	pub woo_commerce: Option<String>,
	pub output: Option<String>,
}

fn request_path(title: &str, default_path: String) -> String {
	let path = tfd::open_file_dialog(title, &default_path, None);
	match path {
		Some(path) => {
			println!("Importing from path: {:?}", path);
			path
		}
		None => panic!("No file path provided"), //TODO - add a dialog box here, for reselection of file path, maybe use this as an opportunity to throw a custom error with a result.
	}
}

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
			request_path("Square", base_path.to_string()),
			request_path("Tickera", base_path.to_string()),
			request_path("WooCommerce", base_path.to_string()),
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
