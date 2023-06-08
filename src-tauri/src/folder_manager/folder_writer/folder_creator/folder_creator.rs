use std::fs;
use std::io::Error;
use std::path::Path;

pub fn create_implementation_folder() -> Result<(), Error> {
    fs::create_dir_all("implementations")
}

pub fn fill_implementation_folder(language_name: String,
                                  version_name: String,
                                  implementation_name: String,
                                  imported_file_names: Vec<String>,
                                  imported_file_paths: Vec<String>,
                                  written_file_names: Vec<String>,
                                  written_file_contents: Vec<String>) {

    let path_str = format!("./implementations/{}/{}/{}", language_name, version_name, implementation_name);
    let path = Path::new(&path_str);
    fs::create_dir_all(path).expect("Unable to create folder");

    // Write the new files
    for i in 0..written_file_names.len() {
        let file_path = path.join(&written_file_names[i]);
        fs::write(file_path, &written_file_contents[i]).expect("unable to create file");
    }

    // Copy the imported files
    for i in 0..imported_file_names.len() {
        let source_path = Path::new(&imported_file_paths[i]);
        let destination_path = path.join(&imported_file_names[i]);
        fs::copy(source_path, destination_path).expect("unable to copy file");
    }

}
