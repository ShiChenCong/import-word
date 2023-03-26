use reqwest::{self, header};
use std::fs::File;

use serde::Deserialize;

#[tauri::command]
pub fn select_file(name: Vec<&str>, key: usize) -> Result<Vec<String>, String> {
    let mut word_vec: Vec<String> = Vec::new();
    let path = name[0];
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

// struct Param {
//     business_id: i32,
//     words: Vec<String>,
// }

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub task_id: String,
}

#[tauri::command]
pub async fn upload_word(token: &str) -> Result<String, String> {
    let client = reqwest::Client::new();

    let url = "https://apiv3.shanbay.com/wordscollection/words_bulk_upload";
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::COOKIE,
        header::HeaderValue::from_str(token).unwrap(),
    );

    let mut word_vec = Vec::new();
    word_vec.push("feast".to_string());
    let param = serde_json::json!({
        "business_id": 6,
        "words": word_vec,
    });
    let result_response = client.post(url).headers(headers).json(&param).send().await;
    match result_response {
        Ok(response) => {
            let json_res = response.json::<ApiResponse>().await;
            match json_res {
                Ok(res) => {
                    println!("{:?}", res);
                    Ok(res.task_id)
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
