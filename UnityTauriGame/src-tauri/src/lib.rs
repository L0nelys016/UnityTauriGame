// src/lib.rs

// DDD Architecture Layers
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;
pub mod usecase;

// WebGL server
pub mod webgl_server;

use std::sync::Arc;

use crate::webgl_server::{WebglServerState};
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
use crate::usecase::{AuthUseCase, GameUseCase, RatingUseCase, GenreUseCase};

/// Глобальный стейт приложения
pub struct AppState {
    pub presentation: PresentationLayer,
    pub webgl_server: WebglServerState,
}

/// Основная точка входа приложения
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // ===== 1. Инициализация базы данных =====
    let data_dir = std::path::PathBuf::from(".data");
    std::fs::create_dir_all(&data_dir).ok();
    let db_path = data_dir.join("unitytaurigame.db");
    let db_path_str = db_path.to_string_lossy().to_string();
    let db = Database::init(&db_path_str).expect("Failed to initialize database");

    // ===== 2. Создание репозиториев =====
    let user_repo: Arc<dyn UserRepository> = Arc::new(SQLiteUserRepository::new(db.clone()));
    let genre_repo: Arc<dyn GenreRepository> = Arc::new(SQLiteGenreRepository::new(db.clone()));
    let game_repo: Arc<dyn GameRepository> = Arc::new(SQLiteGameRepository::new(db.clone()));
    let rating_repo: Arc<dyn UserRatingRepository> = Arc::new(SQLiteUserRatingRepository::new(db.clone()));

    // ===== 3. Создание сервисов =====
    let user_service = Arc::new(UserService::new(user_repo));
    let genre_service = Arc::new(GenreService::new(genre_repo));
    let game_service = Arc::new(GameService::new(game_repo));
    let rating_service = Arc::new(RatingService::new(rating_repo));

    // ===== 4. Создание use cases =====
    let auth_usecase = Arc::new(AuthUseCase::new(user_service.clone()));
    let genre_usecase = Arc::new(GenreUseCase::new(genre_service.clone()));
    let game_usecase = Arc::new(GameUseCase::new(
        game_service.clone(),
        genre_service.clone(),
        rating_service.clone(),
    ));
    let rating_usecase = Arc::new(RatingUseCase::new(rating_service.clone()));

    // ===== 5. Presentation Layer =====
    let presentation = PresentationLayer::new(
        auth_usecase,
        genre_usecase,
        game_usecase,
        rating_usecase,
    );

    // ===== 6. WebGL Server =====
    let webgl_server = WebglServerState::default();

    // ===== 7. Сборка AppState =====
    let app_state = AppState {
        presentation,
        webgl_server,
    };

    // ===== 8. Tauri Builder =====
    webgl_server::webgl_stop().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Presentation commands
            presentation::login,
            presentation::get_all_games,
            presentation::get_game,
            presentation::create_game,
            presentation::update_game,
            presentation::delete_game,
            presentation::search_games,
            presentation::get_games_by_genre,
            presentation::get_all_genres,
            presentation::rate_game,
            presentation::get_game_ratings,
            presentation::get_user_rating_for_game,
            presentation::get_average_rating,
            // WebGL commands
            webgl_server::webgl_start,
            webgl_server::webgl_stop,
            webgl_server::webgl_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
