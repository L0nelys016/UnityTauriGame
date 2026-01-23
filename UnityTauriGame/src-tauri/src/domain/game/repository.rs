use super::entity::Game;
use super::game_title::GameTitle;

pub trait GameRepository {
    fn save(&self, game: &Game) -> Result<(), String>;
    fn find_by_id(&self, id: i64) -> Result<Option<Game>, String>;
    fn find_by_title(&self, title: &GameTitle) -> Result<Option<Game>, String>;
    fn find_all(&self) -> Result<Vec<Game>, String>;
    fn find_by_genre(&self, genre_id: i64) -> Result<Vec<Game>, String>;
    fn find_top_rated(&self, limit: i64) -> Result<Vec<Game>, String>;
    fn delete(&self, id: i64) -> Result<(), String>;
    fn count(&self) -> Result<i64, String>;
}
