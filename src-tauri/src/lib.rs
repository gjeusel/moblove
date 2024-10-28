// use futures::future::join_all;
use scraper::{Html, Selector};
use serde::Serialize;
// use std::error::Error;
use base64;
use tauri_plugin_http::reqwest;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug, Serialize)]
pub struct CommandError {
    message: String,
}

impl From<reqwest::Error> for CommandError {
    fn from(error: reqwest::Error) -> Self {
        CommandError {
            message: error.to_string(),
        }
    }
}

async fn get_manga_chapter_image_urls(url: String) -> Result<Vec<String>, CommandError> {
    let response = reqwest::get(url).await?.text().await?;

    let document = Html::parse_document(&response);
    let selector = Selector::parse("picture img").unwrap();

    let img_urls: Vec<String> = document
        .select(&selector)
        .filter_map(|element| element.value().attr("data-src"))
        .map(|src| src.to_string())
        .collect();

    Ok(img_urls)
}

async fn load_manga_chapter_images(img_urls: &[String]) -> Result<Vec<String>, CommandError> {
    let client = reqwest::Client::new();

    let mut base64_images = Vec::new();

    for img_url in img_urls {
        let img_resp = client
            .get(img_url)
            .header(reqwest::header::REFERER, "https://mangapill.com/")
            .send()
            .await?;

        if img_resp.status().is_success() {
            let bytes = img_resp.bytes().await?;
            let base64_img = format!("data:image/png;base64,{}", base64::encode(&bytes));
            base64_images.push(base64_img);
        }
    }

    Ok(base64_images)
}

#[tauri::command]
async fn get_manga_chapter_images(url: String) -> Result<Vec<String>, CommandError> {
    let img_urls = get_manga_chapter_image_urls(url).await?;
    let base64_images = load_manga_chapter_images(&img_urls).await?;
    Ok(base64_images)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        // .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_manga_chapter_images])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
