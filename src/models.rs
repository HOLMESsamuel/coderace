use std::path::PathBuf;

#[derive(Debug)]
pub struct ImplementationFolder {
    pub path: PathBuf,
    pub name: String
}

pub struct PythonImplementation {
    pub path: PathBuf,
    pub executable_name: String
}

#[derive(Debug)]
pub struct LanguageVersion {
    pub version: String,
    pub implementations: Vec<ImplementationFolder>,
}

#[derive(Debug)]
pub struct Language {
    pub name: String,
    pub versions: Vec<LanguageVersion>,
}

#[derive(Debug)]
pub struct BenchmarkInstructions {
    pub languages: Vec<Language>
}

#[derive(Debug)]
pub struct Result {
    pub language: String,
    pub version: String,
    pub execution_time: String,
    pub memory_usage: String,
    pub image_size: String
}
