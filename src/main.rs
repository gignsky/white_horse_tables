use std::path::PathBuf;
use tinyfiledialogs as tfd;

mod importers;
mod utils;
// mod import::tickera;
// mod import::wooCommerce;

fn main() {
    // println!("{:#?}", &paths);
    // paths.output = Option::Some(String::from(
    //     "/home/gig/local_repos/white_horse_tables/output_files/",
    // ));
    // println!("{:#?}", &paths);
    let paths = get_paths();
    let tables = importers::import_files(paths);
}

fn get_paths() -> utils::FilePaths {
    let square_file_path = path_packer(
        "Square CSV File".to_string(),
        "/home/gig/local_repos/white_horse_tables/source_files/".to_string(),
    );

    let tickera_file_path = path_packer(
        "Tickera CSV File".to_string(),
        "/home/gig/local_repos/white_horse_tables/source_files/".to_string(),
    );

    let woo_commerce_file_path = path_packer(
        "wooCommerce CSV File".to_string(),
        "/home/gig/local_repos/white_horse_tables/source_files/".to_string(),
    );

    utils::FilePaths {
        square: square_file_path,
        tickera: tickera_file_path,
        woo_commerce: woo_commerce_file_path,
        output: None,
    }
}

fn path_packer(
    title: String,        // Title for the dialog box
    default_path: String, // Default path for the dialog box to open to
) -> Option<PathBuf> {
    tfd::open_file_dialog(&title, &default_path, None).map(|path| PathBuf::from(path))
}
