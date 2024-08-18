pub enum Category {
    Bar,
    None,
}

pub enum Item {
    EmployeeBeverage,
    EmployeeSnacks,
    Snacks,
    Drafts,
    Wine,
    CansAndBottles,
    AnyAgeBeverages,
    None,
}

pub enum EventType {
    Payment,
}

pub enum Location {
    WhiteHorseBar,
}

pub struct Customer {
    customer_id: String,
    customer_name: String,
    customer_reference_id: String,
}

pub enum Unit {
    Ea,
}

pub enum ItemType {
    PhysicalItem,
}
