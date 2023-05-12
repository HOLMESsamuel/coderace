use crate::folder_manager::folder_writer::python_folder_writer::write_python_folder;
use crate::models::BenchmarkInstructions;

pub(crate) fn write_folders(benchmark_instructions: &BenchmarkInstructions) -> std::io::Result<()> {
    for language in &benchmark_instructions.languages {
        if language.name.to_lowercase() == "python" {
            match write_python_folder(language) {
                Ok(()) => println!("Python wrappers created successfully."),
                Err(e) => eprintln!("Error writing folders: {}", e),
            }
        }
    }
    Ok(())
}
