// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// DDD Architecture Layers
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

use infrastructure::sqlite::Database;

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
    let _db = Database::init(&db_path_str).expect("Failed to initialize database");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
