// mod lib;

// use dollars::Dollars;
// use lib::{Category, Customer, EventType, Item, ItemType, Location, Unit};
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

// struct Row {
//     name: String,
//     time: String,
//     timezone: String,
//     category: Category,
//     item: Item,
//     quantity: f32,
//     price_point_name: String,
//     sku: String,
//     modifiers_applied: String,
//     gross_sales: Dollars,
//     discounts: Dollars,
//     net_sales: Dollars,
//     tax: Dollars,
//     transaction_id: String,
//     payment_id: String,
//     device_name: String,
//     notes: String,
//     details: String,
//     event_type: EventType,
//     location: Location,
//     dining_option: String,
//     customer: Customer,
//     unit: Unit,
//     count: i32,
//     itemization_type: ItemType,
//     commission: Dollars,
//     employee: String,
//     fulfillment_note: String,
//     token: String,
// }

fn open_csv(filepath: String) -> Result<String, csv::Error> {
    // match std::fs::File::open("contacts.csv") {
    //     Ok(_) => Ok(filepath),
    //     Err(e) => {
    //         let nonsense = Err(csv::Error::from(e));
    //         bail!("Error opening file: {:?}", nonsense);
    //     }
    // }
    Ok(filepath)
}
