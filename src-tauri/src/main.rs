// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod folder_manager;
mod models;
mod command_runner;
mod result_writer;
#[macro_use]
mod race;

use crate::race::race;

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![race])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
