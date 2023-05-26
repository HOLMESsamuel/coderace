#[tauri::command]
pub async fn open_implementation_form_window(app_handle: tauri::AppHandle) {
    let _new_window = tauri::WindowBuilder::new(
        &app_handle,
        "implementation-form",
        tauri::WindowUrl::App("implementation-form.html".into())
    )
        .title("Write implementation")
        .build()
        .expect("failed to build window");
}

#[tauri::command]
pub fn submit_implementation_form() {
    println!("form submitted");
}
