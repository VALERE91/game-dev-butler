use tauri::Manager;
use tokio::task;
use tracing::info;

mod db;
mod models;
mod schema;
mod server;

const API_URL: &str = "localhost:3000";

#[tauri::command]
fn api_url() -> String {
    API_URL.to_string()
}

#[tracing::instrument]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let http_server_join = task::spawn(async {
        server::run(API_URL).await.unwrap();
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                info!("Opening devtools");
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![api_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    http_server_join.await.unwrap();
}
