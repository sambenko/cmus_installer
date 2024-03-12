use std::{fs, io};
use std::path::Path;
use tauri::Window;
use zip::result::ZipError;
use log::info;

pub fn unzip(
    _window: Window,
    file_path: String,
    output_directory: String,
) -> Result<(), ZipError> {
    let file = fs::File::open(&file_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    // Assuming all contents are under a single top-level directory within the ZIP.
    // Extract the name of the top-level directory from the first entry's path.
    let top_level_dir = archive.by_index(0)
        .map_err(|_| ZipError::InvalidArchive("Archive is empty or cannot read first entry"))?
        .enclosed_name()
        .ok_or(ZipError::InvalidArchive("Invalid path in archive"))?
        .components()
        .next()
        .and_then(|c| c.as_os_str().to_str())
        .ok_or(ZipError::InvalidArchive("Cannot identify top-level directory"))?
        .to_owned();

    // Create the top-level directory in the output path
    let output_path = Path::new(&output_directory).join(&top_level_dir);
    fs::create_dir_all(&output_path)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_path = file.enclosed_name().ok_or(ZipError::InvalidArchive("Invalid path in archive"))?;
        
        // Construct the output path for the file
        let outpath = output_path.join(file_path.strip_prefix(&top_level_dir).unwrap_or(file_path));
    
        // Check if the ZIP entry is a directory
        if file.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                fs::create_dir_all(p)?;
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    

    info!("Extraction complete into {}", output_path.to_string_lossy());

    // Cleanup: Delete the ZIP file after successful extraction
    fs::remove_file(&file_path)
        .map_err(|e| {
            info!("Failed to delete the ZIP file: {}", e);
            ZipError::Io(e)
        })?;

    info!("ZIP file deleted successfully.");
    Ok(())
}
