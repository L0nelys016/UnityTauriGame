// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod db;
pub mod dto;
pub mod models;
pub mod commands;

use db::init_database;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize database in .data directory
    let data_dir = std::path::PathBuf::from(".data");
    std::fs::create_dir_all(&data_dir).ok();
    let db_path = data_dir.join("unitytaurigame.db");
    let db_path_str = db_path.to_string_lossy().to_string();
    let _conn = init_database(&db_path_str);
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
