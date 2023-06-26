use tauri::{Manager};
use crate::folder_manager::folder_writer::folder_creator::folder_creator;
use crate::folder_manager::folder_reader::read_implementation_folder_files;
use serde_json::json;
use crate::models::{Config, File};

#[tauri::command]
pub async fn open_implementation_form_window(app_handle: tauri::AppHandle, language_name: String, version_name: String, implementation_name: String) {
    let url = format!("implementation-form.html?language={}&version={}&implementation={}",
                      language_name,
                      version_name,
                      implementation_name
    );
    let _new_window = tauri::WindowBuilder::new(
        &app_handle,
        "implementation-form",
        tauri::WindowUrl::App(url.into())
    )
        .title("Write implementation")
        .inner_size(1080.0, 720.0)
        .build()
        .expect("failed to build window");
}

#[tauri::command]
pub fn submit_implementation_form(language_name: String,
                                  version_name: String,
                                  implementation_name: String,
                                  imported_files_json: String,
                                  written_files_json: String) {
    let imported_files= serde_json::from_str(&imported_files_json).expect("impossible to deserialize");
    let written_files = serde_json::from_str(&written_files_json).expect("impossible to deserialize");
    folder_creator::fill_implementation_folder(language_name, version_name, implementation_name, imported_files, written_files);
}

#[tauri::command]
pub async fn close_implementation_form_window(app_handle: tauri::AppHandle) {
    let window_label = "implementation-form".to_string();
    if let Some(window) = app_handle.get_window(&window_label) {
        window.close().expect("failed to close window");
    }
}

#[tauri::command]
pub async fn load_data(language_name: String, version_name: String, implementation_name: String) -> Result<String, String>{
    let generated_files = vec!["config.json","Dockerfile","requirements.txt","wrapper.py"];

    match read_implementation_folder_files(language_name, version_name, implementation_name) {
        Ok(mut files) => {
            let (method_name, arguments) = if let Some(file) = files.iter().find(|&file| file.name == "config.json") {
                let config:Config = serde_json::from_str(&file.content).expect("impossible to deserialize");
                (Some(config.method_name), Some(config.arguments))
            } else {
                println!("No config.json file");
                (None, None)
            };

            // Filter out generated files and collect back into files
            files = files.into_iter()
                .filter(|file| !generated_files.contains(&file.name.as_str()))
                .collect();

            let json = json!({ "files": files, "methodName": method_name, "arguments": arguments});
            Ok(json.to_string())
        }
        Err(e) => Err(e)
    }
}







