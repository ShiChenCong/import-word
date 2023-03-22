#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use reqwest::{self};

mod util;
use util::import_word;

#[tauri::command]
fn my_custom_command() -> Result<String, String> {
    Err("This failed".into())
}

fn main() {
    tauri::Builder::default()
        // .invoke_handler(tauri::generate_handler![select_file, upload_word])
        // .invoke_handler(tauri::generate_handler![my_custom_command])
        .invoke_handler(tauri::generate_handler![
            import_word::upload_word,
            import_word::select_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
