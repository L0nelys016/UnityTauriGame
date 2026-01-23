use super::game_title::GameTitle;
use super::rating::Rating;

#[derive(Debug, Clone)]
pub struct Game {
    id: i64,
    title: GameTitle,
    description: Option<String>,
    genre_id: i64,
    release_date: String,
    average_rating: Rating,
    total_ratings: i64,
}

impl Game {
    pub fn new(
        id: i64,
        title: GameTitle,
        description: Option<String>,
        genre_id: i64,
        release_date: String,
    ) -> Result<Self, String> {
        Ok(Game {
            id,
            title,
            description,
            genre_id,
            release_date,
            average_rating: Rating::default(),
            total_ratings: 0,
        })
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn title(&self) -> &GameTitle {
        &self.title
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn genre_id(&self) -> i64 {
        self.genre_id
    }

    pub fn release_date(&self) -> &str {
        &self.release_date
    }

    pub fn average_rating(&self) -> Rating {
        self.average_rating.clone()
    }

    pub fn total_ratings(&self) -> i64 {
        self.total_ratings
    }

    pub fn update_rating(&mut self, average: f64, total: i64) -> Result<(), String> {
        self.average_rating = Rating::new(average)?;
        self.total_ratings = total;
        Ok(())
    }

    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
    }
}
