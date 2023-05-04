use std::process::{Command, Output, Stdio};
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

pub(crate) fn run_docker_image(image_name: &str) -> io::Result<Output> {
    let output = Command::new("docker")
        .arg("run")
        .arg(image_name)
        .output()?;

    Ok(output)
}

pub(crate) fn stop_and_remove_docker_container(image_name: &str) -> io::Result<()> {
    // Get the container ID based on the image name
    let container_id_output = Command::new("docker")
        .arg("ps")
        .arg("-a")
        .arg("--filter")
        .arg(format!("ancestor={}", image_name))
        .arg("--format")
        .arg("{{.ID}}")
        .output()?;

    let binding = String::from_utf8_lossy(&container_id_output.stdout);
    let container_id = binding.trim();

    if container_id.is_empty() {
        eprintln!("No container found with the image name: {}", image_name);
        return Ok(());
    }

    let _container_stop = Command::new("docker")
        .arg("stop")
        .arg(container_id)
        //to avoid printing things
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    let _container_remove = Command::new("docker")
        .arg("rm")
        .arg(container_id)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    let _image_remove = Command::new("docker")
        .arg("rmi")
        .arg(image_name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    Ok(())
}

pub(crate) fn get_docker_image_size(image_name: &str) -> io::Result<String> {
    let output = Command::new("docker")
        .arg("image")
        .arg("inspect")
        .arg(image_name)
        .arg("--format")
        .arg("{{.Size}}")
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to get image size for {}: {}", image_name, String::from_utf8_lossy(&output.stderr)),
        ));
    }

    let image_size = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(image_size)
}

pub fn remove_dangling_images() {
    let _remove_dangling_images = Command::new("docker")
        .arg("image")
        .arg("prune")
        .arg("-f")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}
