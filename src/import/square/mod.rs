//! This module is responsible for importing data from Square.

mod lib;

use lib::InitalRow;
// use std::path::Path;

pub fn import(file_path: Option<String>) -> Result<String, csv::Error> {
    // let mut rdr = csv::Reader::from_path(file_path).unwrap();
    // for result in rdr.records() {
    //     let record = result.unwrap();
    //     println!("{:?}", record);
    // }
    // String::from("Square Table")
    let path = match file_path {
        Some(path) => {
            println!("Importing from path: {:?}", path);
            path
        }
        None => panic!("No file path provided"), //TODO - add a dialog box here, for reselection of file path, maybe use this as an opportunity to throw a custom error with a result.
    };
    println!("{:?}", path);
    print_csv(path)
}

fn print_csv(filepath: String) -> Result<String, csv::Error> {
    open_csv(filepath)

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

fn open_csv(filepath: String) -> Result<String, csv::Error> {
    println!("Opening file: {:?}", filepath);
    let mut rdr = csv::Reader::from_path(filepath)?;
    let mut index = 0;
    for result in rdr.deserialize() {
        let record: InitalRow = result?;
        println!("{}: {:?}", index, record);
        index += 1;
    }
    Ok(String::from("Square Table"))
}
