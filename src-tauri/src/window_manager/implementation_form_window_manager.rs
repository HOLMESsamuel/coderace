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
        .build()
        .expect("failed to build window");
}

#[tauri::command]
pub fn submit_implementation_form() {
    println!("form submitted");
}

#[tauri::command]
pub async fn close_implementation_form_window(app_handle: tauri::AppHandle) {
    let window_label = "implementation-form".to_string();
    if let Some(window) = app_handle.get_window(&window_label) {
        window.close().expect("failed to close window");
    }
}
