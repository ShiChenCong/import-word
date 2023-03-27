use futures::future::join_all;
use reqwest::{self, header};
use std::fs::File;

use serde::Deserialize;

pub fn select_file(path: &str, key: usize) -> Result<Vec<String>, String> {
    let mut word_vec: Vec<String> = Vec::new();
    match File::open(path) {
        Ok(file) => {
            let mut rdr = csv::Reader::from_reader(file);
            for result in rdr.records() {
                if let Some(word) = result.unwrap().get(key) {
                    word_vec.push(word.to_string());
                }
            }
            return Ok(word_vec);
        }
        Err(err) => Err(err.to_string()),
    }
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub task_id: String,
}

#[tauri::command]
pub async fn upload_word(
    token: &str,
    file_path: Vec<&str>,
    upload_index: usize,
) -> Result<String, String> {
    let path = file_path[0];
    match select_file(path, upload_index) {
        Ok(words) => {
            let client = reqwest::Client::new();
            let url = "https://apiv3.shanbay.com/wordscollection/words_bulk_upload";
            let mut headers = header::HeaderMap::new();
            headers.insert(
                header::COOKIE,
                header::HeaderValue::from_str(token).unwrap(),
            );

            let mut future_vec = Vec::new();
            let len = words.len();
            if len < 100 {
                let param = serde_json::json!({
                    "business_id": 6,
                    "words": words,
                });
                let result = client
                    .post(url)
                    .headers(headers.clone())
                    .json(&param)
                    .send();
                future_vec.push(result);
            } else {
                for chunk in words.chunks(100) {
                    let param = serde_json::json!({
                        "business_id": 6,
                        "words": chunk,
                    });
                    let result = client
                        .post(url)
                        .headers(headers.clone())
                        .json(&param)
                        .send();
                    future_vec.push(result);
                }
            };
            join_all(future_vec).await;
            Ok(String::from("success"))
        }
        Err(err) => Err(err.to_string()),
    }
}
