// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenvy::dotenv;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // install global collector confi   gured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    game_dev_butler_ui_lib::run().await
}
