// #![warn(missing_docs)]

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;


#[cfg(test)]
mod tests;
mod errors; // We'll put our errors in an `errors` module, and other modules
mod import;
mod utils;
mod config;

// This only gives access within this module. Make this `pub use errors::*;`
// instead if the types must be accessible from other modules (e.g., within
// a `links` section).
#[allow(unused_imports)]
use errors::*;

/// The logger is only initialized in debug mode.
#[cfg(debug_assertions)]
#[allow(dead_code)]
fn init_logger() {
    env_logger::init();
}

#[allow(unused_imports)]
use log::{debug, info, warn, error};

/// The main function, handles the current state of the program.
///
/// This function also handles the error chain and backtrace.
/// it passes off most of the actual work to the `run` function.
fn main() {
    if let Err(ref e) = run(config::TEST_MODE) {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        // Exit with code 1 - general error code, 0 is success
        utils::exit(utils::ExitCode::GeneralError);
    }
}

/// The actual work of the program.
///
/// Runs and if any errors occur it returns a ErrorChain inside a result to the main function.
#[allow(unused_mut)]
fn run(ref test_mode_var: bool) -> Result<()> {
    let paths = match test_mode_var {
        true => utils::get_source_paths(config::BASE_PATH, true),
        false => utils::get_source_paths(config::BASE_PATH, false),
    };
    info!("Inital paths var: {:#?}", &paths);
    // paths.output = Some("/home/gig/local_repos/white_horse_tables/output_files/".to_string());
    // let tables =
    // let path =
    import::import_files(paths);

    Ok(())
}
