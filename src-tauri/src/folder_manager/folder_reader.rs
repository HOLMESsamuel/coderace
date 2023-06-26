use std::{fs, io};
use std::path::{Path, PathBuf};
use crate::models::{BenchmarkInstructions, Config, File, ImplementationFolder, Language, LanguageVersion};
use std::collections::HashMap;
use std::io::Read;

pub fn read_implementation_folder_files(language_name: String, version_name: String, implementation_name: String) -> Result<Vec<File>, String> {
    let path = format!("implementations/{}/{}/{}", language_name, version_name, implementation_name);

    let text_extensions = vec![".txt", ".js", ".html", ".json", ".rs", ".py"];

    match fs::read_dir(Path::new(&path)) {
        Ok(entries) => {
            let mut files = Vec::new();
            for entry in entries {
                if let Ok(entry) = entry {
                    // If the entry is a file, add its name to the list
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            if let Some(file_name) = entry.file_name().to_str() {
                                let content = if text_extensions.iter().any(|ext| file_name.ends_with(ext)) {
                                    // If the file has a text extension, attempt to read it as a text file
                                    let mut file = match fs::File::open(entry.path()) {
                                        Ok(file) => file,
                                        Err(_) => continue,  // Skip this file if it can't be opened
                                    };
                                    let mut content = String::new();
                                    match file.read_to_string(&mut content) {
                                        Ok(_) => Some(content),
                                        Err(_) => None,  // If the file can't be read as text, set its content to None
                                    }
                                } else {
                                    None
                                };
                                let file = File {
                                    path: "".to_string(),
                                    name: file_name.to_string(),
                                    modifiable: !content.is_none(),
                                    content: if content.is_none() {"impossible to read".to_string()} else {content.expect("").to_string()}
                                };
                                files.push(file);
                            }
                        }
                    }
                }
            }
            Ok(files)
        },
        Err(_) => Err(format!("Failed to read directory: {}", path)),
    }
}

pub fn read_implementations_folder() -> io::Result<BenchmarkInstructions> {
    let folder_path = Path::new("implementations");
    if !folder_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "implementations folder not found",
        ));
    }
    let languages = read_language_folder(folder_path)?;
    Ok(BenchmarkInstructions { languages })
}

#[tauri::command]
pub fn read_implementations_folder_for_front() -> Result<String, String> {
    let instructions = match read_implementations_folder() {
        Ok(data) => data,
        Err(e) => return Err(e.to_string()),
    };

    let mut implementations = HashMap::new();

    for language in instructions.languages {
        let mut versions = HashMap::new();
        for version in language.versions {
            let implementation_names = version.implementations.into_iter().map(|imp| imp.name).collect::<Vec<_>>();
            versions.insert(version.version, implementation_names);
        }
        implementations.insert(language.name, versions);
    }

    // Serialize the instructions to a JSON string
    match serde_json::to_string(&implementations) {
        Ok(json) => Ok(json),
        Err(e) => Err(e.to_string()),
    }
}



fn read_language_folder(path: &Path) -> io::Result<Vec<Language>> {
    let mut languages = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            let language_name = path_to_folder_name(&entry_path);
            let language_versions = read_version_folder(&entry_path)?;
            languages.push(Language {
                name: language_name,
                versions: language_versions,
            });
        }
    }

    Ok(languages)
}

fn read_version_folder(path: &Path) -> io::Result<Vec<LanguageVersion>> {
    let mut language_versions = Vec::new();

    for version_entry in fs::read_dir(path)? {
        let version_entry = version_entry?;
        let version_entry_path = version_entry.path();

        if version_entry_path.is_dir() {
            let version_name = path_to_folder_name(&version_entry_path);
            let implementations = read_implementation_folder(&version_entry_path)?;
            language_versions.push(LanguageVersion {
                version: version_name,
                implementations,
            });
        }
    }

    Ok(language_versions)
}

fn read_implementation_folder(path: &Path) -> io::Result<Vec<ImplementationFolder>> {
    let mut implementations = Vec::new();

    for implementation_entry in fs::read_dir(path)? {
        let implementation_entry = implementation_entry?;
        let implementation_entry_path = implementation_entry.path();

        if implementation_entry.path().is_dir() {
            let entries = match fs::read_dir(&implementation_entry_path) {
                Ok(entries) => entries,
                Err(e) => {
                    eprintln!("Error reading directory {}: {}", implementation_entry_path.display(), e);
                    continue;
                }
            };

            if entries.count() > 0 {

                let config_path = implementation_entry_path.join("config.json");
                if !config_path.exists() {
                    eprintln!("config.json not found in folder {}", implementation_entry_path.display());
                    continue;
                }

                let config_content = fs::read_to_string(config_path)?;
                let config: Config = match serde_json::from_str(&config_content) {
                    Ok(config) => config,
                    Err(e) => {
                        eprintln!("Error parsing config.json in folder {}: {}", implementation_entry_path.display(), e);
                        continue;
                    }
                };

                let implementation_folder = ImplementationFolder {
                    name: path_to_folder_name(&implementation_entry_path),
                    path: implementation_entry_path,
                    arguments: config.arguments,
                    method_name: config.method_name,
                    module_name: config.module_name,
                };

                implementations.push(implementation_folder);

            }
        }
    }

    Ok(implementations)
}

fn path_to_folder_name(path: &PathBuf) -> String {
    path.file_name()
        .unwrap()
        .to_string_lossy()
        .to_string()
}
