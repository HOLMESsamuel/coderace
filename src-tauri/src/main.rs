// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod folder_manager;
mod models;
mod command_runner;
mod result_writer;
mod window_manager;
#[macro_use]
mod race;

use std::fs::File;
use std::path::Path;
use crate::race::race;
use crate::window_manager::implementation_form_window_manager::open_implementation_form_window;
use crate::window_manager::implementation_form_window_manager::submit_implementation_form;
use crate::window_manager::implementation_form_window_manager::close_implementation_form_window;
use crate::folder_manager::folder_reader::read_implementations_folder_for_front;
use crate::folder_manager::folder_writer::folder_writer::create_implementation_folder;
use tauri::{CustomMenuItem, Menu, Submenu, Manager};
use folder_manager::folder_writer::folder_creator::folder_creator;
use folder_manager::folder_archiver::folder_archiver;
use tauri::api::dialog;

fn main() {

  let menu = setup_menu();
  
  folder_creator::create_implementation_folder().expect("impossible to create the working folder");

  tauri::Builder::default()
      .menu(menu)
      .invoke_handler(tauri::generate_handler![race,
        read_implementations_folder_for_front,
        create_implementation_folder,
        open_implementation_form_window,
        close_implementation_form_window,
        submit_implementation_form])
      .on_menu_event(|event| {
        match event.menu_item_id() {
          "exit" => {
            event.window().close().unwrap();
          }
          "save" => {
            save()
          }
          "open" => {
            open()
          }
          _ => {}
        }
      })
      .setup(|app| {
        let splashscreen_window = app.get_window("splashscreen").unwrap();
        let main_window = app.get_window("main").unwrap();
        // we perform the initialization code on a new task so the app doesn't freeze
        tauri::async_runtime::spawn(async move {
          std::thread::sleep(std::time::Duration::from_secs(2));
          splashscreen_window.close().unwrap();
          main_window.show().unwrap();
        });
        Ok(())
      })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn setup_menu() -> Menu {
  let exit = CustomMenuItem::new("exit".to_string(), "Exit");
  let save = CustomMenuItem::new("save".to_string(), "Save");
  let open = CustomMenuItem::new("open".to_string(), "Open");
  let file_submenu = Submenu::new("File", Menu::new()
      .add_item(save)
      .add_item(open)
      .add_item(exit));

  let how_does_it_works = CustomMenuItem::new("how does it work ?".to_string(), "How does it work ?");
  let help_submenu = Submenu::new("Help", Menu::new().add_item(how_does_it_works));

  let menu = Menu::new()
      .add_submenu(file_submenu)
      .add_submenu(help_submenu);

  return menu;
}

fn save() {
  // Open a file save dialog
  dialog::FileDialogBuilder::new().save_file(|option| {
    match option {
      Some(mut zip_path) => {
        // Check if the file has a .zip extension, and add it if not
        if zip_path.extension().unwrap_or_default() != "zip" {
          zip_path.set_extension("zip");
        }

        // Open the zip file in write mode
        let file = File::create(&zip_path).expect("Failed to create zip file");
        let mut zip_writer = zip::ZipWriter::new(file);

        let folder_path = Path::new("./implementations");

        // zip the "implementations" directory
        folder_archiver::zip_dir(folder_path, &mut zip_writer).expect("Failed to zip directory");
      },
      None => {
        // The user cancelled the dialog
      },
    }
  });
}

fn open() {
  // Open a file save dialog
  dialog::FileDialogBuilder::new().pick_file(|option| {
    match option {
      Some(zip_path) => {
        folder_archiver::unzip_archive(zip_path).expect("Failed to unzip directory");
      },
      None => {
        // The user cancelled the dialog
      },
    }
  });
}




