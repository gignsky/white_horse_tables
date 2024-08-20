//! Library data and types used by the program.

#[cfg(test)]
mod tests;
mod import;

//
// pub struct ImportedTable {
//     pub source: Source,
//     pub table: [Vec<u64>, Vec<T>]
// }

/// Represents the file paths of the imported csv files and the output file path.
#[derive(Debug)]
pub struct FilePaths {
    ///Path to square csv file
    pub square: Option<String>,
    ///Path to tickera csv file
    pub tickera: Option<String>,
    ///Path to WooCommerce csv file
    pub woo_commerce: Option<String>,
    ///Path to output csv file folder
    pub output: Option<String>,
}
