/// The base path for the resources, this is the path to the resources folder.
///
/// This should be updated to be the downloads folder on whatever type of machine this runs on in the future.
/// TODO! Update this to be the downloads folder on the machine this runs on.
pub const BASE_PATH: &str = "/home/gig/local_repos/white_horse_tables/resources/small-copies/";

/// The test mode flag, if true the program will run in test mode.
pub const TEST_MODE: bool = {
    if cfg!(debug_assertions) {true} else if cfg!(not(debug_assertions)) {false} else {false}
};
