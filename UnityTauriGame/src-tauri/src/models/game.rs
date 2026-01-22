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
