use crate::dto::UserRatingRow;

#[derive(Debug)]
pub struct UserRating {
    pub id: i64,
    pub user_id: i64,
    pub game_id: i64,
    pub rating: i32,
    pub created_at: String,
}

impl From<UserRatingRow> for UserRating {
    fn from(row: UserRatingRow) -> Self {
        UserRating {
            id: row.id,
            user_id: row.user_id,
            game_id: row.game_id,
            rating: row.rating,
            created_at: row.created_at,
        }
    }
}
