//! This module contains the error and exit logic for the program.

/// Controls what exit code to give when the program exits.
///
/// this depends on what error causes the program to exit.
#[derive(thiserror::Error, Debug)]
pub enum Error{
    /// The program exited with a general error that has not been specifically caught. This indicates a bug in the program.
    #[error("Generic Error {0}")]
    Generic(String) = 1,

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

impl Error {
    /// Exits the program with the given exit code.
    ///
    /// The exit code is a number that indicates the reason for the program's exit.
    pub fn exit(code: Error) {
        std::process::exit(code as i32);
    }
}
