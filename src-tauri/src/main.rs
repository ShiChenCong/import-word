#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use reqwest::{self};

mod util;
use util::import_word;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![import_word::upload_word])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
