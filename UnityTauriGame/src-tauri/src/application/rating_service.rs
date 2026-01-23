use std::sync::Arc;
use crate::domain::{UserRating, RatingScore, UserRatingRepository};

pub struct RatingService {
    repository: Arc<dyn UserRatingRepository>,
}

impl RatingService {
    pub fn new(repository: Arc<dyn UserRatingRepository>) -> Self {
        RatingService { repository }
    }

    pub fn rate_game(
        &self,
        id: i64,
        user_id: i64,
        game_id: i64,
        score: i32,
        created_at: String,
    ) -> Result<UserRating, String> {
        let rating_score = RatingScore::new(score)?;
        let rating = UserRating::new(id, user_id, game_id, rating_score, created_at);
        self.repository.save(&rating)?;
        Ok(rating)
    }

    pub fn get_rating(&self, id: i64) -> Result<Option<UserRating>, String> {
        self.repository.find_by_id(id)
    }

    pub fn get_user_game_rating(
        &self,
        user_id: i64,
        game_id: i64,
    ) -> Result<Option<UserRating>, String> {
        self.repository.find_by_user_and_game(user_id, game_id)
    }

    pub fn get_game_ratings(&self, game_id: i64) -> Result<Vec<UserRating>, String> {
        self.repository.find_by_game(game_id)
    }

    pub fn get_user_ratings(&self, user_id: i64) -> Result<Vec<UserRating>, String> {
        self.repository.find_by_user(user_id)
    }

    pub fn delete_rating(&self, id: i64) -> Result<(), String> {
        self.repository.delete(id)
    }

    pub fn get_average_rating(&self, game_id: i64) -> Result<Option<f64>, String> {
        self.repository.get_average_rating(game_id)
    }

    pub fn get_total_ratings(&self, game_id: i64) -> Result<i64, String> {
        self.repository.get_total_ratings(game_id)
    }
}
