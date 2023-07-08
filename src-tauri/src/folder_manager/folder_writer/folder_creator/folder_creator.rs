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

    clean_folder(&imported_files, &written_files, path);

    write_files(written_files, path);
    copy_files(imported_files, path);
}

fn write_files(written_files: Vec<File>, path: &Path) {
    for file in &written_files {
        let file_path = path.join(&file.name);
        if !file.content.is_empty() {
            fs::write(file_path, &file.content).expect("unable to create file");
        }
    }
}

fn copy_files(imported_files: Vec<File>, path: &Path) {
    for file in &imported_files {
        let source_path = Path::new(&file.path);
        let destination_path = path.join(&file.name);
        fs::copy(source_path, destination_path).expect("unable to copy file");
    }
}

fn clean_folder(imported_files: &Vec<File>, written_files: &Vec<File>, path: &Path) {
    let mut updated_files: Vec<String> = imported_files.iter().map(|file| file.name.clone()).collect();
    updated_files.extend(written_files.iter().map(|file| file.name.clone()));

    // Delete all files that are not in updated_files
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                if let Some(file_name_str) = file_name.to_str() {
                    if !updated_files.contains(&file_name_str.to_string()) {
                        fs::remove_file(entry.path()).expect("Unable to remove file");
                    }
                }
            }
        }
    }
}
