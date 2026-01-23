// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// DDD Architecture Layers
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

use std::sync::Arc;

use crate::application::{GameService, GenreService, RatingService, UserService};
use crate::domain::abstractions::{GameRepository, GenreRepository, UserRatingRepository, UserRepository};
use crate::infrastructure::implementation::{
    Database,
    SQLiteGameRepository,
    SQLiteGenreRepository,
    SQLiteUserRatingRepository,
    SQLiteUserRepository,
};
use crate::presentation::PresentationLayer;

pub struct AppState {
    pub presentation: PresentationLayer,
}

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

    // Composition root (manual DI): build repos -> services -> presentation layer
    let db = Database::init(&db_path_str).expect("Failed to initialize database");

    let user_repo: Arc<dyn UserRepository> = Arc::new(SQLiteUserRepository::new(db.clone()));
    let genre_repo: Arc<dyn GenreRepository> = Arc::new(SQLiteGenreRepository::new(db.clone()));
    let game_repo: Arc<dyn GameRepository> = Arc::new(SQLiteGameRepository::new(db.clone()));
    let rating_repo: Arc<dyn UserRatingRepository> = Arc::new(SQLiteUserRatingRepository::new(db.clone()));

    let user_service = Arc::new(UserService::new(user_repo));
    let genre_service = Arc::new(GenreService::new(genre_repo));
    let game_service = Arc::new(GameService::new(game_repo));
    let rating_service = Arc::new(RatingService::new(rating_repo));

    let presentation = PresentationLayer::new(user_service, genre_service, game_service, rating_service);
    let app_state = AppState { presentation };
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
