#[allow(unused_imports)]
use tinyfiledialogs as tfd;

mod import;
mod utils;

#[allow(unused_mut)]
fn main() {
    let mut paths = get_paths();
    // println!("Inital paths var: {:#?}", &paths);
    // paths.output = Some("/home/gig/local_repos/white_horse_tables/output_files/".to_string());
    // let tables =
    import::import_files(paths);
}

fn get_paths() -> utils::FilePaths {
    let square_file_path =
        Some("/home/gig/local_repos/white_horse_tables/resources/small-copies/Square - items-2024-07-15-2024-08-09.csv".to_string());

    let tickera_file_path = Some(
            "/home/gig/local_repos/white_horse_tables/resources/small-copies/Tickera - Attendee List (24).csv".to_string(),
        );

    let woo_commerce_file_path = Some(
            "/home/gig/local_repos/white_horse_tables/resources/small-copies/WooCommerce - wc-orders-report-export-17232178944415.csv".to_string(),
        );

    utils::FilePaths {
        square: square_file_path,             // square_file_path,
        tickera: tickera_file_path,           // tickera_file_path,
        woo_commerce: woo_commerce_file_path, // woo_commerce_file_path,
        output: None,
    }
}
