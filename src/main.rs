mod folder_manager;
mod models;
mod wrapper_writer;
mod dockerfile_writer;

use crate::folder_manager::folder_writer::write_python_wrappers;

fn main() {
    match folder_manager::folder_reader::read_implementations_folder() {
        Ok(benchmark_instructions) => {
            for language in &benchmark_instructions.languages {
                println!("Found language: {}", language.name);
                for version in &language.versions {
                    println!("  Found version: {}", version.version);
                    for implementation in &version.implementations {
                        println!("    Found implementation: {}", implementation.name);
                    }
                }
            }

            // Call the write_python_wrappers function
            match write_python_wrappers(&benchmark_instructions) {
                Ok(()) => println!("Python wrappers created successfully."),
                Err(e) => eprintln!("Error writing Python wrappers: {}", e),
            }
        }
        Err(e) => eprintln!("Error reading implementations folder: {}", e),
    }
}
