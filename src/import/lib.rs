//! A library for working with the all csv file imports

use crate::utils;

/// A table of data from the csv file.
///
/// This represents the table of data from the csv file, either before or after it has been cleaned up.
pub struct Table {
    /// The type of row that the table contains.
    ///
    /// this might be the inital row of data from the csv file, or the final row of data from the csv file.
    pub row_type: RowType,
    /// The source of the table.
    ///
    /// This is the source of the table, in this case it is always.
    pub table_source: utils::Source,
    /// The rows of data in the table.
    pub rows: Vec<RowType>
}

/// An enum representing the types of rows available in the csv file.
pub enum RowType {
    /// The inital row of data from the csv file.
    InitalRow,
    /// The final row of data from the csv file.
    FinalRow,
}

/// Represents a collection of imported csv files in table format.
pub struct MasterImportedTables {
	/// The imported square table
	pub square: Result<Table, csv::Error>,
	// /// The imported tickera table
	// pub tickera: String,
	// /// The imported WooCommerce table
	// pub woo_commerce: String,
}
