use std::{fs, io};
use std::path::{Path, PathBuf};
use crate::models::{BenchmarkInstructions, ImplementationFolder, Language, LanguageVersion};


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
                implementations.push(ImplementationFolder {
                    name: path_to_folder_name(&implementation_entry_path),
                    path: implementation_entry_path,
                });
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
