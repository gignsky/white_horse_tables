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

fn path_packer(title: String, default_path: String) -> Option<PathBuf> {
    let inital_option = tfd::open_file_dialog(&title, &default_path, None);

    match inital_option {
        Some(file_path) => Some(PathBuf::from(file_path)),
        None => None,
    }
}
