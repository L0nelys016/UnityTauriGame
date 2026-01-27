use std::sync::Arc;
use crate::domain::{
    abstractions::GameRepository,
    models::{Game, GameTitle},
};

pub struct GameService {
    repository: Arc<dyn GameRepository>,
}

impl GameService {
    pub fn new(repository: Arc<dyn GameRepository>) -> Self {
        GameService { repository }
    }

    pub fn create_game(
        &self,
        id: i64,
        title: String,
        description: Option<String>,
        genre_id: i64,
        release_date: String,
    ) -> Result<Game, String> {
        let game_title = GameTitle::new(title)?;
        let game = Game::new(id, game_title, description, genre_id, release_date)?;
        self.repository.save(&game)?;
        Ok(game)
    }

    pub fn update_game(
        &self,
        id: i64,
        title: String,
        description: Option<String>,
        genre_id: i64,
        release_date: String,
    ) -> Result<Game, String> {
        // Проверка: существует ли игра
        let existing = self.repository.find_by_id(id)?;
        if existing.is_none() {
            return Err("Игра не найдена".to_string());
        }

        let game_title = GameTitle::new(title)?;
        let game = Game::new(id, game_title, description, genre_id, release_date)?;

        self.repository.update(&game)?;

        Ok(game)
    }

    pub fn get_game(&self, id: i64) -> Result<Option<Game>, String> {
        self.repository.find_by_id(id)
    }

    pub fn get_game_by_title(&self, title: String) -> Result<Option<Game>, String> {
        let game_title = GameTitle::new(title)?;
        self.repository.find_by_title(&game_title)
    }

    pub fn get_all_games(&self) -> Result<Vec<Game>, String> {
        self.repository.find_all()
    }

    pub fn get_games_by_genre(&self, genre_id: i64) -> Result<Vec<Game>, String> {
        self.repository.find_by_genre(genre_id)
    }

    pub fn get_top_rated_games(&self, limit: i64) -> Result<Vec<Game>, String> {
        self.repository.find_top_rated(limit)
    }

    pub fn delete_game(&self, id: i64) -> Result<(), String> {
        self.repository.delete(id)
    }

    pub fn count_games(&self) -> Result<i64, String> {
        self.repository.count()
    }
}
