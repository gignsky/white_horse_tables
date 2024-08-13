use std::path::PathBuf;

#[derive(Debug)]
pub struct FilePaths {
    pub square: Option<PathBuf>,
    pub tickera: Option<PathBuf>,
    pub woo_commerce: Option<PathBuf>,
    pub output: Option<PathBuf>,
}

pub struct MasterImportedTables {
    pub square: String,
    // pub tickera: String,
    // pub woo_commerce: String,
}

enum Source {
    Square,
    Tickera,
    WooCommerce,
}
//
// pub struct ImportedTable {
//     pub source: Source,
//     pub table: [Vec<u64>, Vec<T>]
// }
