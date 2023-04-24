use std::process::{Command, Output};
use std::path::Path;
use std::io;
use crate::models::BenchmarkInstructions;

pub fn build_docker_images(benchmark_instructions: &BenchmarkInstructions) -> io::Result<()> {
    for language in &benchmark_instructions.languages {
        for version in &language.versions {
            for implementation in &version.implementations {
                println!("building image for {} {} {}", language.name, version.version, implementation.name);
                let image_name = format!("{}-{}-{}", language.name, version.version, implementation.name);
                match build_docker_image(&implementation.path, &image_name) {
                    Ok(_) => println!("Docker image for {} {} {} built successfully",language.name, version.version, implementation.name),
                    Err(e) => eprintln!("Error building Docker image for {} {} {}: {}", language.name, version.version, implementation.name, e),
                }
            }
        }
    }
    Ok(())
}

pub fn run_docker_images(benchmark_instructions: &BenchmarkInstructions) -> io::Result<()> {
    for language in &benchmark_instructions.languages {
        for version in &language.versions {
            for implementation in &version.implementations {
                println!("running image {} {} {}", language.name, version.version, implementation.name);
                let image_name = format!("{}-{}-{}", language.name, version.version, implementation.name);
                match run_docker_image(&image_name) {
                    Ok(output) => {
                        let output_str = String::from_utf8_lossy(&output.stdout);
                        println!("Output:\n{}", output_str);
                    }
                    Err(e) => eprintln!("Error running Docker image: {}", e),
                }
            }
        }
    }
    Ok(())
}

fn build_docker_image(folder_path: &Path, image_name: &str) -> io::Result<()> {
    let output = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(image_name)
        .arg(folder_path)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Docker build failed: {}", error_message),
        ))
    }
}

fn run_docker_image(image_name: &str) -> io::Result<Output> {
    let output = Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg(image_name)
        .output()?;

    Ok(output)
}
