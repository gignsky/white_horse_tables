use dollars::Dollars;
use serde::{Deserialize, Deserializer};
use std::path::PathBuf;
use std::{error::Error, io, process};

#[derive(Debug, serde::Deserialize)]
enum Category {
    Bar,
    None,
}

#[derive(Debug, serde::Deserialize)]
enum Item {
    EmployeeBeverage,
    EmployeeSnacks,
    Snacks,
    Drafts,
    Wine,
    CansBtls,
    AnyAgeBeverages,
    None,
}

#[derive(Debug, serde::Deserialize)]
enum EventType {
    Payment,
}

#[derive(Debug, serde::Deserialize)]
enum Location {
    WhiteHorseBar,
}

#[derive(Debug, serde::Deserialize)]
struct Customer {
    customer_id: String,
    customer_name: String,
    customer_reference_id: String,
}

#[derive(Debug, serde::Deserialize)]
enum Unit {
    Ea,
}

#[derive(Debug, serde::Deserialize)]
enum ItemType {
    PhsyicalItem,
}

#[derive(Debug, serde::Deserialize)]
struct SquareRow {
    name: String,
    time: String,
    timezone: String,
    category: Category,
    item: Item,
    quantity: f32,
    price_point_name: String,
    sku: String,
    modifiers_applied: String,
    gross_sales: Dollars,
    discounts: Dollars,
    net_sales: Dollars,
    tax: Dollars,
    transaction_id: String,
    payment_id: String,
    device_name: String,
    notes: String,
    details: String,
    event_type: EventType,
    location: Location,
    dining_option: String,
    customer: Customer,
    unit: Unit,
    count: i32,
    itemization_type: ItemType,
    commission: Dollars,
    employee: String,
    fulfillment_note: String,
    token: String,
}

impl Deserialize<'_> for Dollars {
    fn deserialize<'de, D>(deserializer: D) -> Result<Dollars, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut parts = s.split('.');
        let dollars = parts.next().unwrap_or("0").parse::<i64>().unwrap_or(0);
        let cents = parts.next().unwrap_or("0").parse::<i64>().unwrap_or(0);
        Ok(Dollars::from(dollars * 100 + cents))
    }
}

#[derive(Debug, serde::Deserialize)]
struct UltraRow {
    id: u64,
    row: SquareRow,
}

pub fn import(file_path: Option<PathBuf>) {
    // let mut rdr = csv::Reader::from_path(file_path).unwrap();
    // for result in rdr.records() {
    //     let record = result.unwrap();
    //     println!("{:?}", record);
    // }
    // String::from("Square Table")
    println!("Importing Square Table");
    let path = match file_path {
        Some(path) => path,
        None => panic!("No file path provided"),
    };
    println!("{:?}", path);

    let csv = read_in(path);

    println!("{:?}", csv);
}

fn read_in(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record: SquareRow = result?;
        println!("{:?}", record);
    }
    Ok(())
}
