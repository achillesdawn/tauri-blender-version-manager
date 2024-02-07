use std::{fs, path::PathBuf, str::FromStr};

const BASE_DIR: &'static str = "/home/miguel/blenders/";

#[tauri::command]
pub fn list_dirs(path: String) -> Result<Vec<String>, String> {
    let mut results: Vec<PathBuf> = Vec::with_capacity(10);

    let pathbuf = PathBuf::from_str(&path).unwrap();

    if !pathbuf.exists() {
        return Err("Dir does not Exist".into());
    }

    for entry in fs::read_dir(pathbuf).unwrap() {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();

        if metadata.is_dir() {
            results.push(entry.path());
        } else {
            let path = entry.path();
            let extension = path.extension().unwrap();
            if extension == "xz" {
                results.push(path);
            }
        }
    }

    Ok(results
        .iter()
        .map(|entry| entry.file_name().unwrap().to_str().unwrap().to_owned())
        .collect())
}

#[tauri::command]
pub fn extract_tar_xz(tar_path: String) -> Result<(), String> {
    let mut path = PathBuf::from_str(BASE_DIR).unwrap();
    path.push(tar_path);

    let mut handle = std::process::Command::new("tar")
        .arg("-xf")
        .arg(path.to_str().unwrap())
        .arg("-C")
        .arg(BASE_DIR)
        .spawn()
        .map_err(|err| err.to_string())?;

    handle.wait().unwrap();

    Ok(())
}

#[tauri::command]
pub fn remove_file(file_path: String) -> Result<(), String> {
    let mut path = PathBuf::from_str(BASE_DIR).unwrap();
    path.push(file_path);

    if path.exists() {
        if let Err(error) = fs::remove_file(&path) {
            return Err(error.to_string());
        } else {
            println!("[FS]: Removed {:?}", path);
        }
    }

    Ok(())
}

#[tauri::command]
pub fn remove_dir(dir_path: String) -> Result<(), String> {
    let mut path = PathBuf::from_str(BASE_DIR).unwrap();

    path.push(dir_path);

    dbg!(&path);

    if path.exists() {
        if let Err(error) = fs::remove_dir_all(&path) {
            return Err(error.to_string());
        } else {
            println!("[FS]: Removed Dir {:?}", path);
        }
    } else {
        println!("[FS][dir_path]: Does not exist");
    }
    Ok(())
}
