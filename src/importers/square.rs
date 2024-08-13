use std::path::Path;

pub fn import(file_path: Option<Path>) ->  {
    match file_path {
        Some(path) => {
            println!("Importing Square file: {:?}", file_path);
        }
        None => {
            println!("No file path provided");
        }
    }
    // let mut rdr = csv::Reader::from_path(file_path).unwrap();
    // for result in rdr.records() {
    //     let record = result.unwrap();
    //     println!("{:?}", record);
    // }
    // String::from("Square Table")
}
