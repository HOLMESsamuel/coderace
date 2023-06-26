use std::fs;
use std::io::Error;
use std::path::Path;
use crate::models::File;

pub fn create_implementation_folder() -> Result<(), Error> {
    fs::create_dir_all("implementations")
}

pub fn fill_implementation_folder(language_name: String,
                                  version_name: String,
                                  implementation_name: String,
                                  imported_files: Vec<File>,
                                  written_files: Vec<File>) {

    let path_str = format!("./implementations/{}/{}/{}", language_name, version_name, implementation_name);
    let path = Path::new(&path_str);
    fs::create_dir_all(path).expect("Unable to create folder");

    println!("folder created");

    // Write the new files
    for i in 0..written_files.len() {
        println!("write file");
        let file_path = path.join(&written_files[i].name);
        fs::write(file_path, &written_files[i].content).expect("unable to create file");
    }

    // Copy the imported files
    for i in 0..imported_files.len() {
        let source_path = Path::new(&imported_files[i].path);
        let destination_path = path.join(&imported_files[i].name);
        fs::copy(source_path, destination_path).expect("unable to copy file");
    }

}
