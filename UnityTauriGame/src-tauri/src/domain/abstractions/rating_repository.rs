use crate::domain::models::UserRating;

pub trait UserRatingRepository: Send + Sync {
    fn save(&self, rating: &UserRating) -> Result<(), String>;
    fn find_by_id(&self, id: i64) -> Result<Option<UserRating>, String>;
    fn find_by_user_and_game(&self, user_id: i64, game_id: i64) -> Result<Option<UserRating>, String>;
    fn find_by_game(&self, game_id: i64) -> Result<Vec<UserRating>, String>;
    fn find_by_user(&self, user_id: i64) -> Result<Vec<UserRating>, String>;
    fn delete(&self, id: i64) -> Result<(), String>;
    fn delete_by_user_and_game(&self, user_id: i64, game_id: i64) -> Result<(), String>;
    fn get_average_rating(&self, game_id: i64) -> Result<Option<f64>, String>;
    fn get_total_ratings(&self, game_id: i64) -> Result<i64, String>;
}
