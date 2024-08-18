use crate::utils::FilePaths;
use crate::utils::MasterImportedTables;

mod square;
// mod tickera;
// mod woo_commerce;

pub fn import_files(paths: FilePaths) {
    // let square_table =
    square::import(paths.square);
    // let tickera_table = import_tickera_table(paths.tickera);
    // let woo_commerce_table = import_woo_commerce_table(paths.woo_commerce);

    // MasterImportedTables {
    //     square: square_table,
    //     // tickera: tickera_table,
    //     // woo_commerce: woo_commerce_table,
    // }
}
