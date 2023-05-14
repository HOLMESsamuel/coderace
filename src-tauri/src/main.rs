// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod folder_manager;
mod models;
mod command_runner;
mod result_writer;
#[macro_use]
mod race;

use std::fs::File;
use std::path::Path;
use crate::race::race;
use tauri::{CustomMenuItem, Menu, Submenu};
use folder_manager::folder_writer::folder_creator::folder_creator;
use folder_manager::folder_archiver::folder_archiver;
use tauri::api::dialog;

fn main() {

  let menu = setup_menu();
  
  folder_creator::create_implementation_folder().expect("impossible to create the working folder");

  tauri::Builder::default()
      .menu(menu)
      .invoke_handler(tauri::generate_handler![race])
      .on_menu_event(|event| {
        match event.menu_item_id() {
          "quit" => {
            std::process::exit(0);
          }
          "close" => {
            event.window().close().unwrap();
          }
          "export" => {
            export()
          }
          _ => {}
        }
      })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn setup_menu() -> Menu {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let close = CustomMenuItem::new("close".to_string(), "Close");
  let export = CustomMenuItem::new("export".to_string(), "Export");
  let file_submenu = Submenu::new("File", Menu::new()
      .add_item(export)
      .add_item(quit)
      .add_item(close));

  let how_does_it_works = CustomMenuItem::new("how does it work ?".to_string(), "How does it work ?");
  let help_submenu = Submenu::new("Help", Menu::new().add_item(how_does_it_works));

  let menu = Menu::new()
      .add_submenu(file_submenu)
      .add_submenu(help_submenu);

  return menu;
}

fn export() {
  // Choose the directory where the zip file will be saved
  dialog::FileDialogBuilder::new().pick_folder(|option|{
    match option {
      Some(dir_path) => {
        // Create the path for the zip file
        let zip_path = dir_path.join("implementations.zip");

        // Open the zip file in write mode
        let file = File::create(&zip_path).expect("Failed to create zip file");
        let mut zip_writer = zip::ZipWriter::new(file);

        let folder_path = Path::new("./implementations");

        // Call your function to zip the "implementations" directory
        folder_archiver::zip_dir(folder_path, &mut zip_writer).expect("Failed to zip directory");
      },
      None => {
        // The user cancelled the dialog
      },
    }
  });
}


