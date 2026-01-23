#[derive(Debug, Clone)]
pub struct RatingEntity {
    pub id: i64,
    pub user_id: i64,
    pub game_id: i64,
    pub score: i32,
    pub created_at: String,
}

impl RatingEntity {
    pub fn new(id: i64, user_id: i64, game_id: i64, score: i32, created_at: String) -> Self {
        RatingEntity {
            id,
            user_id,
            game_id,
            score,
            created_at,
        }
    }
}
