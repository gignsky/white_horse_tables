// // `error_chain!` can recurse deeply
// #![recursion_limit = "1024"]
//
// // Import the macro. Don't forget to add `error-chain` in your
// // `Cargo.toml`!
// #[macro_use]
// extern crate error_chain;
//
// // We'll put our errors in an `errors` module, and other modules in
// // this crate will `use errors::*;` to get access to everything
// // `error_chain!` creates.
// mod errors {
//     error_chain! {} // Create the Error, ErrorKind, ResultExt, and Result types
// }
//
// // This only gives access within this module. Make this `pub use errors::*;`
// // instead if the types must be accessible from other modules (e.g., within
// // a `links` section).
// #[allow(unused_imports)]
// use errors::*;

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
