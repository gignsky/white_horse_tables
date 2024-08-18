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
struct FilePaths {
    square: Option<String>,
    tickera: Option<String>,
    woo_commerce: Option<String>,
    output: Option<String>,
}

pub fn get_source_paths(test_mode: Option<bool>) -> FilePaths {
    match test_mode {
        Some(true) => {
            let square_file_path =
                Some("/home/gig/local_repos/white_horse_tables/resources/small-copies/Square - items-2024-07-15-2024-08-09.csv".to_string());

            let tickera_file_path = Some(
                    "/home/gig/local_repos/white_horse_tables/resources/small-copies/Tickera - Attendee List (24)".to_string(),
                );

            let woo_commerce_file_path = Some(
                    "/home/gig/local_repos/white_horse_tables/resources/small-copies/WooCommerce - wc-orders-report-export-17232178944415.csv".to_string(),
                );

            FilePaths {
                square: square_file_path,             // square_file_path,
                tickera: tickera_file_path,           // tickera_file_path,
                woo_commerce: woo_commerce_file_path, // woo_commerce_file_path,
                output: None,
            }
        }
        _ => {
            let square_file_path = Some(request_path(
                "Square",
                "/home/gig/local_repos/white_horse_tables/resources/small-copies".to_string(),
            ));

            let tickera_file_path = Some(request_path(
                "Tickera",
                "/home/gig/local_repos/white_horse_tables/resources/small-copies".to_string(),
            ));

            let woo_commerce_file_path = Some(request_path(
                "WooCommerce",
                "/home/gig/local_repos/white_horse_tables/resources/small-copies".to_string(),
            ));

            FilePaths {
                square: square_file_path,             // square_file_path,
                tickera: tickera_file_path,           // tickera_file_path,
                woo_commerce: woo_commerce_file_path, // woo_commerce_file_path,
                output: None,
            }
        }
    }
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
