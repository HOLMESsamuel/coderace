use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::write::FileOptions;
use zip::CompressionMethod::Stored;
use walkdir::WalkDir;
use zip::ZipArchive;

pub fn zip_dir(path: &Path, zip_file: &mut zip::ZipWriter<File>) -> zip::result::ZipResult<()> {
    let options = FileOptions::default()
        .compression_method(Stored)
        .unix_permissions(0o755);

    let walkdir = WalkDir::new(path);
    let it = walkdir.into_iter();

    for dent in it {
        let dent = dent.unwrap();

        let path = dent.path();
        let name = path.strip_prefix(Path::new(".")).unwrap();

        if path.is_file() {
            zip_file.start_file_from_path(name, options)?;
            let mut f = File::open(&path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip_file.write_all(&buffer)?;
        } else if name.as_os_str() != "." {
            zip_file.add_directory_from_path(name, options)?;
        }
    }
    Ok(())
}

//replace the current implementation folder with the one extracted from the archive on given path
pub fn unzip_archive(archive_path: PathBuf) -> Result<(), std::io::Error> {
    let archive_file = File::open(&archive_path)?;
    let mut archive = ZipArchive::new(archive_file)?;

    // Create the path for the new "implementations" directory
    let new_dir_path = Path::new("implementations");

    // If the "implementations" directory already exists, delete it
    if new_dir_path.exists() {
        std::fs::remove_dir_all(&new_dir_path)?;
    }

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = new_dir_path.join(file.name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}




