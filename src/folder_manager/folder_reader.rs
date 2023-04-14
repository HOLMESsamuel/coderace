use std::{fs, io};
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Implementation {
    path: PathBuf,
    name: String
}

#[derive(Debug)]
struct LanguageVersion {
    version: String,
    implementations: Vec<Implementation>,
}

#[derive(Debug)]
struct Language {
    name: String,
    versions: Vec<LanguageVersion>,
}

fn read_folder_recursive(path: &Path) -> io::Result<Vec<Language>> {
    let mut languages = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            let language_name = path_to_folder_name(&entry_path);

            let mut language_versions = Vec::new();

            for version_entry in fs::read_dir(entry_path)? {
                let version_entry = version_entry?;
                let version_entry_path = version_entry.path();

                if version_entry_path.is_dir() {
                    let version_name = path_to_folder_name(&version_entry_path);

                    let mut implementations = Vec::new();

                    for implementation_entry in fs::read_dir(version_entry_path)? {
                        let implementation_entry = implementation_entry?;
                        let implementation_entry_path = implementation_entry.path();

                        if implementation_entry.path().is_dir() {
                            implementations.push(Implementation {
                                name: path_to_folder_name(&implementation_entry_path),
                                path: implementation_entry_path

                            });
                        }
                    }

                    language_versions.push(LanguageVersion {
                        version: version_name,
                        implementations,
                    });
                }
            }

            languages.push(Language {
                name: language_name,
                versions: language_versions,
            });
        }
    }

    Ok(languages)
}

pub fn read_implementations_folder() -> io::Result<()> {
    let folder_path = Path::new("implementations");

    if !folder_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "implementations folder not found",
        ));
    }

    let languages = read_folder_recursive(folder_path)?;

    for language in &languages {
        println!("Found language: {}", language.name);
        for version in &language.versions {
            println!("  Found version: {}", version.version);
            for implementation in &version.implementations {
                println!("    Found implementation: {}", implementation.name);
            }
        }
    }

    Ok(())
}

fn path_to_folder_name(path: &PathBuf) -> String {
    path.file_name()
        .unwrap()
        .to_string_lossy()
        .to_string()
}
