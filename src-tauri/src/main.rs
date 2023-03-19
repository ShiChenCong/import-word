#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use reqwest;
use serde_json::json;
use std::{collections::HashMap, fs::File};

#[tauri::command]
fn select_file(name: Vec<&str>) -> Result<Vec<String>, String> {
    let mut word_vec: Vec<String> = Vec::new();
    // 获取第几列 先假设为第一列
    let path = name[0];
    match File::open(path) {
        Ok(file) => {
            let mut rdr = csv::Reader::from_reader(file);
            for result in rdr.records() {
                if let Some(word) = result.unwrap().get(1) {
                    word_vec.push(word.to_string());
                }
            }
            return Ok(word_vec);
        }
        Err(err) => Err(err.to_string()),
    }
}

struct Param {
    business_id: i32,
    words: Vec<String>,
}

#[tauri::command]
async fn upload_word() -> Result<(), String> {
    let url = "https://apiv3.shanbay.com/wordscollection/words_bulk_upload";
    let client = reqwest::Client::new();
    let mut word_vec = Vec::new();
    word_vec.push("feast".to_string());
    let param = serde_json::json!({
        "business_id": 6,
        "words": word_vec,
    });
    client.post(url).json(&param).send().await.unwrap();
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![select_file, upload_word])
        // .invoke_handler(tauri::generate_handler![upload_word])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
