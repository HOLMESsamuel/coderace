use std::fs;
use std::path::Path;
use crate::folder_manager::folder_writer::python_folder_writer::write_python_folder;
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
    }
    Ok(())
}
