use std::fs::File;
use std::io::prelude::*;
use crate::models::{BenchmarkInstructions, LanguageVersion, PythonImplementation};
use crate::wrapper_writer::python_wrapper_writer;
use crate::dockerfile_writer::python_dockerfile_writer;

pub(crate) fn write_python_wrappers(benchmark_instructions: &BenchmarkInstructions) -> std::io::Result<()> {
    for language in &benchmark_instructions.languages {
        if language.name.to_lowercase() == "python" {
            for version in &language.versions {
                for implementation in &version.implementations {
                    let python_implementation = PythonImplementation {
                        path: implementation.path.clone(),
                        executable_name: "python".to_string(),
                    };
                    write_python_dockerfile(&python_implementation, &version)?;
                    write_python_wrapper_file(&python_implementation)?;
                    //TODO : ajouter un requirements.txt si il n'existe pas
                    //TODO : recuperer le nom de la methode
                }
            }
        }
    }
    Ok(())
}

fn write_python_dockerfile(python_implementation: &PythonImplementation, version: &LanguageVersion) -> std::io::Result<()> {
    let wrapper_path = python_implementation.path.join("Dockerfile");
    let mut file = File::create(wrapper_path)?;

    let python_dockerfile = python_dockerfile_writer::write_python_dockerfile(&version.version);

    file.write_all(python_dockerfile.as_bytes())?;
    Ok(())
}

fn write_python_wrapper_file(python_implementation: &PythonImplementation) -> std::io::Result<()> {
    let wrapper_path = python_implementation.path.join("wrapper.py");
    let mut file = File::create(wrapper_path)?;

    let wrapper_code = python_wrapper_writer::write_python_wrapper("factorial");

    file.write_all(wrapper_code.as_bytes())?;
    Ok(())
}
