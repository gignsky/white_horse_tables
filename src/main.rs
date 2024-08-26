#![allow(unused_imports, dead_code)]

use csv;
use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// struct SquareImportedRow {
//     date: String,
//     time: String,
//     timezone: String,
//     category: String,
//     item: String,
//     quantity: f32,
//     price_point_name: String,
//     sku: String,
//     modifiers_applied: String,
//     gross_sales: String,
//     discounts: String,
//     net_sales: String,
//     tax: String,
//     transaction_id: String,
//     payment_id: String,
//     device_name: String,
//     notes: String,
//     details: String,
//     event_type: String,
//     location: String,
//     dining_option: String,
//     customer_id: String,
//     customer_name: String,
//     customer_reference_id: String,
//     unit: String,
//     count: i32,
//     itemization_type: String,
//     commission: String,
//     employee: String,
//     fulfillment_note: String,
//     token: String,
// }

#[derive(Debug, Deserialize)]
struct SquareImportedRow {
    item_one: String,
    item_two: String,
    item_three: String,
}

fn main() {
    println!("-------------------------------------------------------------------------------------------------------------------------");

    // let path = "/home/gig/local_repos/white_horse_tables/resources/small-copies/Square - items-2024-07-15-2024-08-09.csv".to_string();
    let path = "/home/gig/local_repos/white_horse_tables/resources/small-copies/test.csv".to_string();

    let table: Vec<SquareImportedRow> = Vec::new();


    print!("{:?}", table);
}
