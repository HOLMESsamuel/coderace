use std::path::PathBuf;
use serde_derive::Deserialize;

#[derive(Debug)]
pub struct ImplementationFolder {
    pub path: PathBuf,
    pub name: String,
    pub arguments: Vec<Argument>,
    pub method_name: String,
    pub module_name: String
}

#[derive(Debug, Deserialize)]
pub struct Argument {
    pub name: String,
    pub value: String,
    pub argument_type: String
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub method_name: String,
    pub module_name: String,
    pub arguments: Vec<Argument>
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
