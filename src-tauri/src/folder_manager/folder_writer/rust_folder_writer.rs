use std::fs::File;
use std::io::Write;
use crate::models::{ImplementationFolder, Language, LanguageVersion};
use crate::folder_manager::folder_writer::folder_writer::write_in_file_if_not_exist;
use crate::folder_manager::folder_writer::wrapper_writer::rust_wrapper_writer::write_rust_wrapper;
use crate::folder_manager::folder_writer::dockerfile_writer;

pub(crate) fn write_rust_folder(language: &Language) -> std::io::Result<()> {
    for version in &language.versions {
        for implementation in &version.implementations {
            write_rust_cargo_file(implementation)?;
            write_rust_wrapper_file(implementation)?;
            write_rust_dockerfile(implementation, version)?;
        }
    }
    Ok(())
}

fn write_rust_cargo_file(implementation: &ImplementationFolder) -> std::io::Result<()> {
    let requirements_path = implementation.path.join("Cargo.toml");
    //the bin lines declares the main method as wrapper.rs instead of main.rs by default
    let dependencies = vec![
        "[package]",
        "name = \"wrapper\"",
        "version = \"0.1.0\"",
        "edition = \"2021\"",
        "[[bin]]",
        "name = \"wrapper\"",
        "path = \"wrapper.rs\"",
        "[dependencies]",
        "time = \"0.1\"",
        "sys-info = \"0.5\""
    ];

    write_in_file_if_not_exist(requirements_path, dependencies)
}

fn write_rust_wrapper_file(rust_implementation: &ImplementationFolder) -> std::io::Result<()> {
    let wrapper_path = rust_implementation.path.join("wrapper.rs");
    let mut file = File::create(wrapper_path)?;

    let wrapper_code = write_rust_wrapper(rust_implementation);

    file.write_all(wrapper_code.as_bytes())?;
    Ok(())
}

fn write_rust_dockerfile(
    implementation: &ImplementationFolder,
    version: &LanguageVersion,
) -> std::io::Result<()> {
    let wrapper_path = implementation.path.join("Dockerfile");
    let mut file = File::create(wrapper_path)?;

    let rust_dockerfile = dockerfile_writer::rust_dockerfile_writer::write_rust_dockerfile(&version.version);

    file.write_all(rust_dockerfile.as_bytes())?;
    Ok(())
}
