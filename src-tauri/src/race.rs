use crate::folder_manager::folder_writer::folder_writer::write_folders;
use crate::command_runner::docker_command::build_docker_images;
use crate::command_runner::docker_command::remove_dangling_images;
use crate::command_runner::docker_command::is_docker_running;
use crate::folder_manager;
use crate::result_writer::result_writer::run_docker_images;
use tauri::Window;

#[tauri::command]
pub async fn race(window: Window) -> Result<String, String>{

    let log = |message: &str| {
        window.emit("LOG", message).unwrap();
    };

    match folder_manager::folder_reader::read_implementations_folder() {
        Ok(benchmark_instructions) => {
            match write_folders(&benchmark_instructions) {
                Ok(()) => log("wrappers created successfully"),
                Err(e) => eprintln!("Error writing folders: {}", e),
            };
            if is_docker_running() {
                log("building docker images...");
                match build_docker_images(&benchmark_instructions) {
                    Ok(()) => println!("docker images built"),
                    Err(e) => eprintln!("Error building docker images: {}", e),
                };
                log("running docker images...");
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
                Ok("Race completed!".to_string())
            } else {
                Ok("Docker is not installed or not running".to_string())
            }

        }

        Err(e) => {
            return Err("Error reading folder: ".to_string() + &*e.to_string());
        }
    }

}
