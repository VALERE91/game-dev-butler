use tauri::Manager;
use tokio::task;
use tracing::info;

mod server;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tracing::instrument]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let http_server_join = task::spawn(async {
        server::run().await;
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    http_server_join.await.unwrap();
}
