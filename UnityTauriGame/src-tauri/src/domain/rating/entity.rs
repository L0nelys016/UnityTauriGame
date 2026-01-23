use super::rating_score::RatingScore;

#[derive(Debug, Clone)]
pub struct UserRating {
    id: i64,
    user_id: i64,
    game_id: i64,
    score: RatingScore,
    created_at: String,
}

impl UserRating {
    pub fn new(id: i64, user_id: i64, game_id: i64, score: RatingScore, created_at: String) -> Self {
        UserRating {
            id,
            user_id,
            game_id,
            score,
            created_at,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn game_id(&self) -> i64 {
        self.game_id
    }

    pub fn score(&self) -> RatingScore {
        self.score
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn change_score(&mut self, score: RatingScore) {
        self.score = score;
    }
}
