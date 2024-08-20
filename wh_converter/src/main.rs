// #![warn(missing_docs)]

use log::debug;
use wh_api as api;
use wh_shared_lib as lib;

#[cfg(test)]
mod tests;
// mod errors; // We'll put our errors in an `errors` module, and other modules
mod config;
mod exits;
mod utils;

/// The main function, handles the current state of the program.
///
/// This function also handles the error chain and backtrace.
/// it passes off most of the actual work to the `run` function.
///
/// TODO! ErrorChain is deprecated, and needs to be removed!!!
fn main() {
//     if let Err(ref e) = run(config::TEST_MODE) {
//         use std::io::Write;
//         let stderr = &mut ::std::io::stderr();
//         let errmsg = "Error writing to stderr";
//
//         writeln!(stderr, "error: {}", e).expect(errmsg);
//
//         for e in e.iter().skip(1) {
//             writeln!(stderr, "caused by: {}", e).expect(errmsg);
//         }
//
//         // The backtrace is not always generated. Try to run this example
//         // with `RUST_BACKTRACE=1`.
//         if let Some(backtrace) = e.backtrace() {
//             writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
//         }
//
//         // Exit with code 1 - general error code, 0 is success
//         exits::ExitCode::GeneralError.exit();
//     }
    run(config::TEST_MODE);
}

/// The actual work of the program.
///
/// Runs and if any errors occur it returns a ErrorChain inside a result to the main function.
#[allow(unused_mut)]
fn run(ref test_mode_var: bool) {
    // let paths = match test_mode_var {
    //     true => utils::get_source_paths(config::BASE_PATH, true),
    //     false => utils::get_source_paths(config::BASE_PATH, false),
    // };
    let paths = utils::get_source_paths(config::BASE_PATH, test_mode_var.clone());
    debug!("Inital paths var: {:#?}", &paths);
    println!("Inital paths var: {:#?}", &paths);
    // paths.output = Some("/home/gig/local_repos/white_horse_tables/output_files/".to_string());
    // let tables =
    // let path =
    api::import::import_files(paths);
}
