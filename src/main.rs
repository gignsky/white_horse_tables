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

// This only gives access within this module. Make this `pub use errors::*;`
// instead if the types must be accessible from other modules (e.g., within
// a `links` section).
#[allow(unused_imports)]
use errors::*;

#[allow(unused_mut)]
fn main() {
    if let Err(ref e) = run() {
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

fn run() -> Result<()> {
    let mut paths = utils::get_source_paths(Some(true));
    println!("Inital paths var: {:#?}", &paths);
    // paths.output = Some("/home/gig/local_repos/white_horse_tables/output_files/".to_string());
    // let tables =
    // let path = import::import_files(paths);

    Ok(())
}
