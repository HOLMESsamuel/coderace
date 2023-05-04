mod folder_manager;
mod models;
mod wrapper_writer;
mod dockerfile_writer;
mod command_runner;
mod result_writer;

use crate::folder_manager::folder_writer::folder_writer::write_folders;
use crate::command_runner::docker_command::build_docker_images;
use crate::command_runner::docker_command::remove_dangling_images;
use crate::result_writer::result_writer::run_docker_images;

fn main() {
    match folder_manager::folder_reader::read_implementations_folder() {
        Ok(benchmark_instructions) => {
            match write_folders(&benchmark_instructions) {
                Ok(()) => println!("wrappers created successfully."),
                Err(e) => eprintln!("Error writing folders: {}", e),
            };
            println!("building docker images...");
            match build_docker_images(&benchmark_instructions) {
                Ok(()) => println!("docker images built"),
                Err(e) => eprintln!("Error building docker images: {}", e),
            };
            println!("running docker images...");
            match run_docker_images(&benchmark_instructions) {
                Ok(r) => {
                    for result in r {
                        println!("Benchmark results for {}.{} :",result.language, result.version);
                        println!("execution time : {} s", result.execution_time);
                        println!("memory usage : {} Mb", result.memory_usage);
                        println!("image size : {} bytes", result.image_size);
                        println!();
                    }
                },
                Err(e) => eprintln!("Error running docker images: {}", e),
            }
            remove_dangling_images();
        }
        Err(e) => eprintln!("Error reading implementations folder: {}", e),
    }
}
