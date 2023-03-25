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
    // headers.insert(header::COOKIE, header::HeaderValue::from_static("_ga=GA1.2.1455716351.1676722123; sensorsdata2015jssdkcross=%7B%22distinct_id%22%3A%22yuhkav%22%2C%22first_id%22%3A%22186646d3053966-0289844b609a63-1f525634-3686400-186646d30541312%22%2C%22props%22%3A%7B%22%24latest_traffic_source_type%22%3A%22%E8%87%AA%E7%84%B6%E6%90%9C%E7%B4%A2%E6%B5%81%E9%87%8F%22%2C%22%24latest_search_keyword%22%3A%22%E6%9C%AA%E5%8F%96%E5%88%B0%E5%80%BC%22%2C%22%24latest_referrer%22%3A%22https%3A%2F%2Fwww.google.com%2F%22%7D%2C%22%24device_id%22%3A%22186646d3053966-0289844b609a63-1f525634-3686400-186646d30541312%22%7D; _gat=1; auth_token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6MjE0Mzk5MDY4LCJleHAiOjE2ODA1ODM5OTksImV4cF92MiI6MTY4MDU4Mzk5OSwiZGV2aWNlIjoiIiwidXNlcm5hbWUiOiJXZWNoYXRfYjE4NDAxZTkzNTBmZTAwMyIsImlzX3N0YWZmIjowLCJzZXNzaW9uX2lkIjoiOTE4YmZlNWViZmJkMTFlZGJlMjIwYTdkOGMwYmUwMGYifQ.4-zE-EtGiu4hDi3WUqKEOZBhkFtNu--GpD9DGk7kj8E; csrftoken=620aa48c4a15ed98eaafb93fc545d0f8"));

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
