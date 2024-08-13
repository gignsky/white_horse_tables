use dollars::Dollars;
use std::path::PathBuf;

enum Category {
    Bar,
    None,
}

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

enum EventType {
    Payment,
}

enum Location {
    WhiteHorseBar,
}

struct Customer {
    customer_id: String,
    customer_name: String,
    customer_reference_id: String,
}

enum Unit {
    Ea,
}

enum ItemType {
    PhsyicalItem,
}

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

    
}
