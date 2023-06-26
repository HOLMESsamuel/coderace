use std::path::PathBuf;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ImplementationFolder {
    pub path: PathBuf,
    pub name: String,
    pub arguments: Vec<Argument>,
    pub method_name: String,
    pub module_name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub path: String,
    pub name: String,
    pub content: String,
    pub modifiable: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Argument {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageVersion {
    pub version: String,
    pub implementations: Vec<ImplementationFolder>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub versions: Vec<LanguageVersion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkInstructions {
    pub languages: Vec<Language>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub language: String,
    pub version: String,
    pub name: String,
    pub execution_time: String,
    pub memory_usage: String,
    pub image_size: String
}
