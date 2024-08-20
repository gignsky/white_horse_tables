//! This module is responsible for importing data from Square.

mod lib;

use lib::{InitalRow, unpack_csv};
use log::{debug, info};
use crate::import::lib::Table;
// use std::path::Path;

pub fn import(file_path: Option<String>) -> Result<Table, csv::Error> {
    // let mut rdr = csv::Reader::from_path(file_path).unwrap();
    // for result in rdr.records() {
    //     let record = result.unwrap();
    //     println!("{:?}", record);
    // }
    // String::from("Square Table")
    let path = match file_path {
        Some(path) => {
            info!("Importing from path: {:?}", path);
            path
        }
        None => panic!("No file path provided"), //TODO - add a dialog box here, for reselection of file path, maybe use this as an opportunity to throw a custom error with a result.
    };
    debug!("Square path to import: {:?}", path);
    print_csv(path)
}

fn print_csv(filepath: String) -> Result<Table, csv::Error> {
    let unpacked = lib::unpack_csv(filepath);
    unpacked

    //     // Print headers
    //     if let Some(headers) = rdr.headers()?.iter().next() {
    //         println!("{}", headers);
    //     }
    //
    //     // Iterate over each record.
    //     for result in rdr.records() {
    //         let record = result?;
    //         println!("{}", record.iter().collect::<Vec<&str>>().join(", "));
    //     }
    //
    //     Ok(())
}
