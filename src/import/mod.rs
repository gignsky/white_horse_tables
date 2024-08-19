//! This module is responsible for importing the CSV files into the program.

use crate::utils::FilePaths;
use crate::utils::MasterImportedTables;

mod square;
// mod tickera;
// mod woo_commerce;

/// Imports the CSV files from the provided file paths.
pub fn import_files(paths: FilePaths) -> Result<MasterImportedTables, csv::Error> {
    let square_import = square::import(paths.square);
    // let tickera_table = import_tickera_table(paths.tickera)?;
    // let woo_commerce_table = import_woo_commerce_table(paths.woo_commerce)?;

    let import_master = MasterImportedTables {
        square: square_import,
        // tickera: tickera_table,
        // woo_commerce: woo_commerce_table,
    };

    Ok(import_master)
}
