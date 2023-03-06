#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use actix_cors::Cors;
use actix_web::{http, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: i32,
    name: String,
}

#[post("/")]
async fn index(body: web::Json<User>) -> Result<impl Responder, Error> {
    let obj = User {
        id: body.id,
        name: body.name.to_string(),
    };
    Ok(web::Json(obj))
}

fn main() {
    tauri::Builder::default()
        .setup(|_| {
            tauri::async_runtime::spawn({
                HttpServer::new(|| {
                    let cors = Cors::default()
                        .allow_any_origin()
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST"])
                        .allowed_header(http::header::CONTENT_TYPE);
                    App::new().wrap(cors).service(index)
                })
                .bind(("127.0.0.1", 8080))?
                .run()
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![select_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
