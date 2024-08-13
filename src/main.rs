use std::path::Path;
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
        "Square CSV File",
        "/home/gig/local_repos/white_horse_tables/source_files/",
    );

    let tickera_file_path = path_packer(
        "Tickera CSV File",
        "/home/gig/local_repos/white_horse_tables/source_files/",
    );

    let woo_commerce_file_path = path_packer(
        "wooCommerce CSV File",
        "/home/gig/local_repos/white_horse_tables/source_files/",
    );

    utils::FilePaths {
        square: square_file_path,
        tickera: tickera_file_path,
        woo_commerce: woo_commerce_file_path,
        output: None,
    }
}

fn path_packer(title: str, default_path: str) -> Option<Path> {
    let inital_option = tfd::open_file_dialog(title, default_path, None);

    match inital_option {
        Some(path) => {
            let path = Path::new(&path.clone());
            Some(path)
        }
        None => None,
    }
}
