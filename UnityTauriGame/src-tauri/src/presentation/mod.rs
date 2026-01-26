use std::sync::Arc;
use crate::usecase::{AuthUseCase, GameUseCase, RatingUseCase, GenreUseCase};

pub struct PresentationLayer {
    pub auth_usecase: Arc<AuthUseCase>,
    pub genre_usecase: Arc<GenreUseCase>,
    pub game_usecase: Arc<GameUseCase>,
    pub rating_usecase: Arc<RatingUseCase>,
}

impl PresentationLayer {
    pub fn new(
        auth_usecase: Arc<AuthUseCase>,
        genre_usecase: Arc<GenreUseCase>,
        game_usecase: Arc<GameUseCase>,
        rating_usecase: Arc<RatingUseCase>,
    ) -> Self {
        PresentationLayer {
            auth_usecase,
            genre_usecase,
            game_usecase,
            rating_usecase,
        }
    }
}

// Tauri Commands

#[tauri::command]
pub async fn login(
    username: String,
    password: String,
    state: tauri::State<'_, crate::AppState>,
) -> Result<crate::usecase::auth_usecase::UserDto, String> {
    state.presentation.auth_usecase.login(username, password)
}

#[tauri::command]
pub async fn get_all_games(
    state: tauri::State<'_, crate::AppState>,
) -> Result<Vec<crate::usecase::game_usecase::GameDto>, String> {
    state.presentation.game_usecase.get_all_games()
}

#[tauri::command]
pub async fn get_game(
    id: i64,
    state: tauri::State<'_, crate::AppState>,
) -> Result<Option<crate::usecase::game_usecase::GameDto>, String> {
    state.presentation.game_usecase.get_game(id)
}

#[tauri::command]
pub async fn create_game(
    title: String,
    description: Option<String>,
    genre_id: i64,
    release_date: String,
    state: tauri::State<'_, crate::AppState>,
) -> Result<crate::usecase::game_usecase::GameDto, String> {
    state.presentation.game_usecase.create_game(title, description, genre_id, release_date)
}

#[tauri::command]
pub async fn update_game(
    id: i64,
    title: String,
    description: Option<String>,
    genre_id: i64,
    release_date: String,
    state: tauri::State<'_, crate::AppState>,
) -> Result<crate::usecase::game_usecase::GameDto, String> {
    state
        .presentation
        .game_usecase
        .update_game(id, title, description, genre_id, release_date)
}

#[tauri::command]
pub async fn delete_game(
    id: i64,
    state: tauri::State<'_, crate::AppState>,
) -> Result<(), String> {
    state.presentation.game_usecase.delete_game(id)
}

#[tauri::command]
pub async fn search_games(
    query: String,
    state: tauri::State<'_, crate::AppState>,
) -> Result<Vec<crate::usecase::game_usecase::GameDto>, String> {
    state.presentation.game_usecase.search_games(query)
}

#[tauri::command]
pub async fn get_games_by_genre(
    genre_id: i64,
    state: tauri::State<'_, crate::AppState>,
) -> Result<Vec<crate::usecase::game_usecase::GameDto>, String> {
    state.presentation.game_usecase.get_games_by_genre(genre_id)
}

#[tauri::command]
pub async fn get_all_genres(
    state: tauri::State<'_, crate::AppState>,
) -> Result<Vec<crate::usecase::genre_usecase::GenreDto>, String> {
    state.presentation.genre_usecase.get_all_genres()
}

#[tauri::command]
pub async fn rate_game(
    user_id: i64,
    game_id: i64,
    score: i32,
    state: tauri::State<'_, crate::AppState>,
) -> Result<crate::usecase::rating_usecase::RatingDto, String> {
    state.presentation.rating_usecase.rate_game(user_id, game_id, score)
}

#[tauri::command]
pub async fn get_game_ratings(
    game_id: i64,
    state: tauri::State<'_, crate::AppState>,
) -> Result<Vec<crate::usecase::rating_usecase::RatingDto>, String> {
    state.presentation.rating_usecase.get_game_ratings(game_id)
}

#[tauri::command]
pub async fn get_user_rating_for_game(
    user_id: i64,
    game_id: i64,
    state: tauri::State<'_, crate::AppState>,
) -> Result<Option<crate::usecase::rating_usecase::RatingDto>, String> {
    state.presentation.rating_usecase.get_user_rating_for_game(user_id, game_id)
}

#[tauri::command]
pub async fn get_average_rating(
    game_id: i64,
    state: tauri::State<'_, crate::AppState>,
) -> Result<f64, String> {
    state.presentation.rating_usecase.get_average_rating(game_id)
}
