use std::fs;
use std::io::Error;

pub fn create_implementation_folder() -> Result<(), Error> {
    fs::create_dir_all("implementations")
}
