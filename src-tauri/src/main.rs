#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use actix_web::{get, web, App, HttpServer, Responder};
use std::fs::File;

#[tauri::command]
fn select_file(name: Vec<&str>) -> Result<Vec<String>, String> {
    let mut word_vec: Vec<String> = Vec::new();
    // 获取第几列
    let path = name[0];
    match File::open(path) {
        Ok(file) => {
            let mut rdr = csv::Reader::from_reader(file);
            for result in rdr.records() {
                match result {
                    Ok(record) => {
                        if let Some(word) = record.get(1) {
                            word_vec.push(word.to_string());
                        }
                    }
                    Err(err) => return Err(err.to_string()),
                }
            }
            return Ok(word_vec);
        }
        Err(err) => Err(err.to_string()),
    }
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

fn main() {
    tauri::Builder::default()
        .setup(|_| {
            tauri::async_runtime::spawn({
                HttpServer::new(|| App::new().service(greet))
                    .bind(("127.0.0.1", 8080))?
                    .run()
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![select_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
