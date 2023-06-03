use tauri::Manager;

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
                                  imported_file_names: Vec<String>,
                                  imported_file_paths: Vec<String>,
                                  written_file_names: Vec<String>,
                                  written_file_contents: Vec<String>) {
    println!("{}", written_file_names[0]);
    println!("{}", written_file_names[1]);
    println!("{}", written_file_contents[0]);
    println!("{}", written_file_contents[1]);
}

#[tauri::command]
pub async fn close_implementation_form_window(app_handle: tauri::AppHandle) {
    let window_label = "implementation-form".to_string();
    if let Some(window) = app_handle.get_window(&window_label) {
        window.close().expect("failed to close window");
    }
}
