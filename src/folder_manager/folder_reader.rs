use std::{fs, io};
use std::path::Path;

pub fn read_implementations_folder() -> io::Result<()> {
    let folder_path = Path::new("implementations");

    if !folder_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "implementations folder not found",
        ));
    }

    let entries = fs::read_dir(folder_path)?;

    for entry in entries {
        let entry = entry?;
        println!("Found: {}", entry.path().display());
    }

    Ok(())
}

