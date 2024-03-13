
use std::fs;
use std::path::{ Path, PathBuf };

#[tauri::command]
pub async fn cleanup(version: String, target_path: String) -> Result<String, String> {
    let path = Path::new(&target_path).join(format!("cmus-{}", version)); 
    match fs::remove_dir_all(path) {
        Ok(_) => Ok("Files cleaned up successfully".into()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn copy_dir(from: &str, to: &str) -> Result<(), String> {
    let from_path = Path::new(from);
    let to_path = PathBuf::from(to);

    if !from_path.is_dir() {
        return Err(format!("{} is not a directory", from));
    }

    fs::create_dir_all(&to_path)
        .map_err(|err| format!("Failed to create target directory: {}", err))?;

    for entry in fs::read_dir(from_path).map_err(|err| format!("Failed to read source directory: {}", err))? {
        let entry = entry.map_err(|err| format!("Failed to read directory entry: {}", err))?;
        let from_entry = entry.path();
        let to_entry = to_path.join(entry.file_name());

        if entry.file_type().map_err(|err| format!("Failed to get file type: {}", err))?.is_dir() {
            copy_dir(&from_entry.to_string_lossy(), &to_entry.to_string_lossy())?;
        } else {
            fs::copy(&from_entry, &to_entry)
                .map_err(|err| format!("Failed to copy file: {}", err))?;
        }
    }

    Ok(())
}