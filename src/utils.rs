use std::path::Path;

#[derive(Debug)]
pub struct FilePaths {
    pub square: Option<Path>,
    pub tickera: Option<Path>,
    pub woo_commerce: Option<Path>,
    pub output: Option<Path>,
}

pub struct MasterImportedTables {
    pub square: String,
    pub tickera: String,
    pub woo_commerce: String,
}
