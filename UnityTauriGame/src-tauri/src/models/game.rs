use crate::dto::GameRow;
use crate::models::Genre;

#[derive(Debug)]
pub struct Game {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub genre: Genre,
    pub release_date: String,
    pub average_rating: f64,
    pub total_ratings: i64,
}

impl From<GameRow> for Game {
    fn from(row: GameRow) -> Self {
        Game {
            id: row.game_id,
            title: row.title,
            description: row.description,
            release_date: row.release_date,
            average_rating: row.average_rating,
            total_ratings: row.total_ratings,
            genre: Genre {
                id: row.genre_id,
                name: row.genre_name,
            },
        }
    }
}
