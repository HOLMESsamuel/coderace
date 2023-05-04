use std::collections::HashSet;
use crate::dockerfile_writer::python_dockerfile_writer;
use crate::models::{
    ImplementationFolder, Language, LanguageVersion, PythonImplementation,
};
use crate::wrapper_writer::python_wrapper_writer;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;

pub(crate) fn write_python_folder(language: &Language) -> std::io::Result<()> {
    for version in &language.versions {
        for implementation in &version.implementations {
            let python_implementation = PythonImplementation {
                path: implementation.path.clone(),
                executable_name: "python".to_string(),
            };
            write_python_dockerfile(&python_implementation, &version)?;
            write_python_wrapper_file(&python_implementation)?;
            write_python_requirement_file(implementation)?;
            //TODO : recuperer le nom de la methode
        }
    }
    Ok(())
}

fn write_python_dockerfile(
    python_implementation: &PythonImplementation,
    version: &LanguageVersion,
) -> std::io::Result<()> {
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

//Add a requirements.txt file to the folder in order to install all the required dependencies
//"memory-profiler" dependency is mandatory to run the benchmark it is added whatever the code
fn write_python_requirement_file(implementation: &ImplementationFolder) -> std::io::Result<()> {
    let requirements_path = implementation.path.join("requirements.txt");
    let dependencies = vec!["memory-profiler", "six"];

    if !requirements_path.exists() {
        // Create requirements.txt file if it doesn't exist
        let mut file = fs::File::create(&requirements_path)?;
        for dependency in &dependencies {
            writeln!(file, "{}", dependency)?;
        }
    } else {
        // Read the contents of the existing requirements.txt file
        let file = fs::File::open(&requirements_path)?;
        let reader = BufReader::new(file);

        let mut existing_dependencies = HashSet::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                existing_dependencies.insert(line.trim().to_string());
            }
        }

        let mut requirements_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&requirements_path)?;

        // Append the required dependencies that are not already in the file
        for dependency in &dependencies {
            if !existing_dependencies.contains(*dependency) {
                writeln!(requirements_file, "{}", dependency)?;
            }
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_write_python_wrapper_file() -> std::io::Result<()> {
        // Create a temporary directory
        let temp_dir = tempdir()?;
        let temp_dir_path = temp_dir.path().to_path_buf();

        let python_implementation = PythonImplementation {
            path: temp_dir_path.clone(),
            executable_name: "python".to_string(),
        };

        // Call the write_python_wrapper_file function
        write_python_wrapper_file(&python_implementation)?;

        // Check if the wrapper.py file exists
        let wrapper_path = temp_dir_path.join("wrapper.py");
        assert!(wrapper_path.exists());

        // Read the contents of the wrapper.py file
        let contents = fs::read_to_string(wrapper_path)?;

        // Check if the contents of the file match the expected output
        let expected_output = python_wrapper_writer::write_python_wrapper("factorial");
        assert_eq!(contents, expected_output);

        Ok(())
    }
}

