#[derive(Debug, Clone)]
pub struct GameEntity {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub genre_id: i64,
    pub release_date: String,
    pub rating: f64,
}

impl GameEntity {
    pub fn new(
        id: i64,
        title: String,
        description: Option<String>,
        genre_id: i64,
        release_date: String,
        rating: f64,
    ) -> Self {
        GameEntity {
            id,
            title,
            description,
            genre_id,
            release_date,
            rating,
        }
    }
}
