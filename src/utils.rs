#[derive(Debug)]
pub struct FilePaths {
    pub square: Option<String>,
    pub tickera: Option<String>,
    pub woo_commerce: Option<String>,
    pub output: Option<String>,
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
