use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDto {
    pub id: i64,
    pub username: String,
    pub role: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub role: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenreDto {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGenreRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameDto {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub genre_id: i64,
    pub release_date: String,
    pub average_rating: f64,
    pub total_ratings: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGameRequest {
    pub title: String,
    pub description: Option<String>,
    pub genre_id: i64,
    pub release_date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RatingDto {
    pub id: i64,
    pub user_id: i64,
    pub game_id: i64,
    pub score: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRatingRequest {
    pub user_id: i64,
    pub game_id: i64,
    pub score: i32,
}
