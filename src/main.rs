#[allow(unused_imports)]
use tinyfiledialogs as tfd;

mod import;
mod utils;

#[allow(unused_mut)]
fn main() {
    let mut paths = utils::get_source_paths(Some(true));
    println!("Inital paths var: {:#?}", &paths);
    // paths.output = Some("/home/gig/local_repos/white_horse_tables/output_files/".to_string());
    // let tables =
    // let path = import::import_files(paths);
}
