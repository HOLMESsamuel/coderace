use std::io;
use crate::models::BenchmarkInstructions;
use crate::models::BenchmarkResult;
use crate::command_runner::docker_command;

pub fn run_docker_images(benchmark_instructions: &BenchmarkInstructions) -> io::Result<Vec<BenchmarkResult>> {
    let mut results: Vec<BenchmarkResult> = Vec::new();
    for language in &benchmark_instructions.languages {
        for version in &language.versions {
            for implementation in &version.implementations {
                println!("running image {} {} {}", language.name, version.version, implementation.name);
                let image_name = format!("{}-{}-{}", language.name, version.version, implementation.name);
                match docker_command::run_docker_image(&image_name) {
                    Ok(output) => {
                        let output_str = String::from_utf8_lossy(&output.stdout);
                        let lines: Vec<&str> = output_str.lines().collect();
                        let mut result: BenchmarkResult = BenchmarkResult {
                            language: language.name.to_string(),
                            version: version.version.to_string(),
                            name: implementation.name.to_string(),
                            execution_time: lines[1].to_string(),
                            memory_usage: lines[2].to_string(),
                            image_size: "".to_string(),
                        };
                        match docker_command::get_docker_image_size(&image_name) {
                            Ok(s) => result.image_size = s,
                            Err(e) => println!("{}", e)
                        }
                        println!("stopping and removing {} image", &image_name);
                        match docker_command::stop_and_remove_docker_container(&image_name) {
                            Ok(()) => println!("image removed"),
                            Err(e) => println!("{}", e)
                        }
                        results.push(result);
                    }
                    Err(e) => eprintln!("Error running Docker image: {}", e),
                }
            }
        }
    }
    Ok(results)
}
