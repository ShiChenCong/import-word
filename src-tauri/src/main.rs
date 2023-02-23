#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::File;

#[tauri::command]
fn greet(name: Vec<&str>) {
    let mut word_vec: Vec<String> = Vec::new();
    let path = name[0];
    let mut rdr = csv::Reader::from_reader(File::open(path).unwrap());
    for result in rdr.records() {
        let record = result.unwrap();
        word_vec.push(record.get(1).unwrap().to_string());
    }
    println!("{:?}", word_vec);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
