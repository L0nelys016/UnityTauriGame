use std::sync::Arc;
use crate::application::RatingService;

pub struct RatingUseCase {
    rating_service: Arc<RatingService>,
}

impl RatingUseCase {
    pub fn new(rating_service: Arc<RatingService>) -> Self {
        RatingUseCase { rating_service }
    }

    pub fn rate_game(
        &self,
        user_id: i64,
        game_id: i64,
        score: i32,
    ) -> Result<RatingDto, String> {
        // Check if user already rated this game - update if exists
        if let Some(existing) = self.rating_service
            .get_user_game_rating(user_id, game_id)
            .map_err(|e| format!("Failed to check existing rating: {}", e))? 
        {
            // For now, delete old and create new (in production, implement proper update)
            let _ = self.rating_service.delete_rating(existing.id());
        }

        // Generate new ID by getting all ratings and finding max
        let all_ratings = self.rating_service
            .get_game_ratings(game_id)
            .map_err(|e| format!("Failed to get ratings: {}", e))?;
        
        let new_id = all_ratings
            .iter()
            .map(|r| r.id())
            .max()
            .unwrap_or(0) + 1;
        
        let created_at = chrono::Utc::now().to_rfc3339();

        let rating = self.rating_service.rate_game(
            new_id,
            user_id,
            game_id,
            score,
            created_at.clone(),
        )?;

        Ok(RatingDto {
            id: rating.id(),
            user_id: rating.user_id(),
            game_id: rating.game_id(),
            score: rating.score().as_i32(),
            created_at: rating.created_at().to_string(),
        })
    }

    pub fn get_game_ratings(&self, game_id: i64) -> Result<Vec<RatingDto>, String> {
        let ratings = self.rating_service.get_game_ratings(game_id)?;
        Ok(ratings
            .into_iter()
            .map(|r| RatingDto {
                id: r.id(),
                user_id: r.user_id(),
                game_id: r.game_id(),
                score: r.score().as_i32(),
                created_at: r.created_at().to_string(),
            })
            .collect())
    }

    pub fn get_user_rating_for_game(
        &self,
        user_id: i64,
        game_id: i64,
    ) -> Result<Option<RatingDto>, String> {
        match self.rating_service.get_user_game_rating(user_id, game_id)? {
            Some(rating) => Ok(Some(RatingDto {
                id: rating.id(),
                user_id: rating.user_id(),
                game_id: rating.game_id(),
                score: rating.score().as_i32(),
                created_at: rating.created_at().to_string(),
            })),
            None => Ok(None),
        }
    }

    pub fn get_average_rating(&self, game_id: i64) -> Result<f64, String> {
        Ok(self.rating_service
            .get_average_rating(game_id)?
            .unwrap_or(0.0))
    }

    pub fn get_total_ratings(&self, game_id: i64) -> Result<i64, String> {
        self.rating_service.get_total_ratings(game_id)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RatingDto {
    pub id: i64,
    pub user_id: i64,
    pub game_id: i64,
    pub score: i32,
    pub created_at: String,
}
