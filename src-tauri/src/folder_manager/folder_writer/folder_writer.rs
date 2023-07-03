use std::collections::HashSet;
use std::fs;
use std::fs::{OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::io::Write;
use crate::folder_manager::folder_writer::python_folder_writer::write_python_folder;
use crate::folder_manager::folder_writer::rust_folder_writer::write_rust_folder;
use crate::models::BenchmarkInstructions;

#[tauri::command]
pub fn create_implementation_folder(language_name: String, version_name: String, implementation_name: String) -> Result<String, String> {
    let path_string = format!("./implementations/{}/{}/{}", language_name, version_name, implementation_name);
    let path = Path::new(&path_string);
    if path.exists() {
        Err("The implementation folder already exists".to_string())
    } else {
        fs::create_dir_all(&path).expect("error creating the folders");
        Ok("Folder created".to_string())
    }
}

#[tauri::command]
pub fn delete_implementation_folder(language_name: String, version_name: String, implementation_name: String) -> Result<String, String> {
    let path_string = format!("./implementations/{}/{}/{}", language_name, version_name, implementation_name);
    let path = Path::new(&path_string);
    if !path.exists() {
        Err("The implementation folder does not exist".to_string())
    } else {
        fs::remove_dir_all(&path).expect("error deleting the folders");

        // remove version directory if it is empty
        let version_dir = path.parent().unwrap();
        if version_dir.read_dir().expect("read dir call failed").count() == 0 {
            fs::remove_dir(&version_dir).expect("failed to remove directory");
        }

        // remove language directory if it is empty
        let language_dir = version_dir.parent().unwrap();
        if language_dir.read_dir().expect("read dir call failed").count() == 0 {
            fs::remove_dir(&language_dir).expect("failed to remove directory");
        }

        Ok("Folder deleted".to_string())
    }
}

pub(crate) fn write_folders(benchmark_instructions: &BenchmarkInstructions) -> std::io::Result<()> {
    for language in &benchmark_instructions.languages {
        if language.name.to_lowercase() == "python" {
            match write_python_folder(language) {
                Ok(()) => println!("Python wrappers created successfully."),
                Err(e) => eprintln!("Error writing folders: {}", e),
            }
        }
        if language.name.to_lowercase() == "rust" {
            match write_rust_folder(language) {
                Ok(()) => println!("Rust wrappers created successfully."),
                Err(e) => eprintln!("Error writing folders: {}", e),
            }
        }
    }
    Ok(())
}

pub fn write_in_file_if_not_exist(requirements_path: PathBuf, dependencies: Vec<&str>) -> std::io::Result<()>{
    if !requirements_path.exists() {
        //file if it doesn't exist
        let mut file = fs::File::create(&requirements_path)?;
        for dependency in &dependencies {
            writeln!(file, "{}", dependency)?;
        }
    } else {
        // Read the contents of the existing file
        let file = fs::File::open(&requirements_path)?;
        let reader = BufReader::new(file);

        let mut existing_dependencies = HashSet::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                existing_dependencies.insert(line.trim().to_string());
            }
        }

        let mut requirements_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&requirements_path)?;

        // Append the required dependencies that are not already in the file
        for dependency in &dependencies {
            if !existing_dependencies.contains(*dependency) {
                writeln!(requirements_file, "{}", dependency)?;
            }
        }
    }

    Ok(())
}
