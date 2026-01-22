#[derive(Debug)]
pub struct GameRow {
    pub game_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub release_date: String,
    pub average_rating: f64,
    pub total_ratings: i64,

    pub genre_id: i64,
    pub genre_name: String,
}
