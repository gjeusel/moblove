// use tauri_plugin_http::reqwest;
// use serde_json::Value;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// async fn fetch_neatpush() -> Result<Value, String> {
//     let url = "https://messy.s3.fr-par.scw.cloud/neatpush.json";
//     let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
//     let json: Value = response.json().await.map_err(|e| e.to_string())?;
//     Ok(json)
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
