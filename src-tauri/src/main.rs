// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod requests;
mod paths;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            requests::get_download_links,
            requests::download,
            paths::list_dirs,
            paths::extract_tar_xz,
            paths::remove_file,
            paths::remove_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
 