mod folder_manager;
mod models;
mod wrapper_writer;
mod dockerfile_writer;
mod command_runner;

use crate::folder_manager::folder_writer::folder_writer::write_folders;
use crate::command_runner::docker_command;

fn main() {
    match folder_manager::folder_reader::read_implementations_folder() {
        Ok(benchmark_instructions) => {
            match write_folders(&benchmark_instructions) {
                Ok(()) => println!("wrappers created successfully."),
                Err(e) => eprintln!("Error writing folders: {}", e),
            };
            println!("building docker images...");
            match docker_command::build_docker_images(&benchmark_instructions) {
                Ok(()) => println!("docker images built"),
                Err(e) => eprintln!("Error building docker images: {}", e),
            };
            println!("running docker images...");
            match docker_command::run_docker_images(&benchmark_instructions) {
                Ok(()) => println!("docker images runned"),
                Err(e) => eprintln!("Error running docker images: {}", e),
            }
        }
        Err(e) => eprintln!("Error reading implementations folder: {}", e),
    }
}
