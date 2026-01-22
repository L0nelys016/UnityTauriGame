#[derive(Debug)]
pub struct UserRating {
    pub id: i64,
    pub user_id: i64,
    pub game_id: i64,
    pub rating: i32,
    pub created_at: String,
}
