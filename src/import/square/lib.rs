//! This module contains the data structures that are used to deserialize the Square API response.

use dollars::Dollars;
use serde::Deserialize;

/// A row of data from the Square csv file represented as a struct.
///
/// This represents the inital row of data from the Square csv file, prior to it being cleaned up.
#[derive(Debug, Deserialize)]
pub struct InitalRow {
    date: String,
    time: String,
    timezone: String,
    category: String,
    item: String,
    quantity: f32,
    price_point_name: String,
    sku: String,
    modifiers_applied: String,
    gross_sales: f32,
    discounts: f32,
    net_sales: f32,
    tax: f32,
    transaction_id: String,
    payment_id: String,
    device_name: String,
    notes: String,
    details: String,
    event_type: String,
    location: String,
    dining_option: String,
    customer: String,
    unit: String,
    count: i32,
    itemization_type: String,
    commission: f32,
    employee: String,
    fulfillment_note: String,
    token: String,
}

// ///This struct represents the final row of data from the Square csv file
// ///
// ///This represents the final row of data from the Square csv file, after it has been cleaned up.
// struct FinalRow {
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
    CansAndBottles,
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
    PhysicalItem,
}
