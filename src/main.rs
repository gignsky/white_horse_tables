#[allow(unused_imports)]
use tinyfiledialogs as tfd;

mod importers;
mod utils;

#[allow(unused_mut)]
fn main() {
    // println!("{:#?}", &paths);
    // paths.output = Option::Some(String::from(
    //     "/home/gig/local_repos/white_horse_tables/output_files/",
    // ));
    // println!("{:#?}", &paths);
    let mut paths = get_paths();
    // println!("{:#?}", &paths);
    // let tables =
    importers::import_files(paths);
}

fn get_paths() -> utils::FilePaths {
    let square_file_path =
        Some("/home/gig/local_repos/white_horse_tables/source_files/".to_string());

    //     let tickera_file_path = path_packer(
    //         "/home/gig/local_repos/white_horse_tables/source_files/".to_string(),
    //     );
    //
    //     let woo_commerce_file_path = path_packer(
    //         "/home/gig/local_repos/white_horse_tables/source_files/".to_string(),
    //     );

    utils::FilePaths {
        square: square_file_path,
        tickera: None,      // tickera_file_path,
        woo_commerce: None, // woo_commerce_file_path,
        output: None,
    }
}
