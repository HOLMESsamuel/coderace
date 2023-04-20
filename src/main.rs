mod folder_manager;
mod models;
mod wrapper_writer;
mod dockerfile_writer;

use crate::folder_manager::folder_writer::folder_writer::write_folders;

fn main() {
    match folder_manager::folder_reader::read_implementations_folder() {
        Ok(benchmark_instructions) => {
            match write_folders(&benchmark_instructions) {
                Ok(()) => println!("wrappers created successfully."),
                Err(e) => eprintln!("Error writing folders: {}", e),
            }
        }
        Err(e) => eprintln!("Error reading implementations folder: {}", e),
    }
}
