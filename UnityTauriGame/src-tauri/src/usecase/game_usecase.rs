use std::sync::Arc;
use crate::application::{GameService, GenreService, RatingService};
use crate::domain::models::Game;

pub struct GameUseCase {
    game_service: Arc<GameService>,
    genre_service: Arc<GenreService>,
    rating_service: Arc<RatingService>,
}

impl GameUseCase {
    pub fn new(
        game_service: Arc<GameService>,
        genre_service: Arc<GenreService>,
        rating_service: Arc<RatingService>,
    ) -> Self {
        GameUseCase {
            game_service,
            genre_service,
            rating_service,
        }
    }

    pub fn create_game(
        &self,
        title: String,
        description: Option<String>,
        genre_id: i64,
        release_date: String,
    ) -> Result<GameDto, String> {
        // Проверяем, существует ли жанр
        self.genre_service
            .get_genre(genre_id)?
            .ok_or_else(|| "Жанр не найден".to_string())?;

        // Генерация ID (простейший вариант)
        let count = self.game_service.count_games()?;
        let new_id = count + 1;

        let game = self.game_service.create_game(
            new_id,
            title,
            description,
            genre_id,
            release_date,
        )?;

        Ok(self.game_to_dto(&game)?)
    }

    pub fn update_game(
        &self,
        id: i64,
        title: String,
        description: Option<String>,
        genre_id: i64,
        release_date: String,
    ) -> Result<GameDto, String> {
        // Проверяем, существует ли жанр
        self.genre_service
            .get_genre(genre_id)?
            .ok_or_else(|| "Жанр не найден".to_string())?;

        // Обновляем игру через game_service
        let game = self.game_service.update_game(id, title, description, genre_id, release_date)?;
        
        Ok(self.game_to_dto(&game)?)
    }

    pub fn get_all_games(&self) -> Result<Vec<GameDto>, String> {
        let games = self.game_service.get_all_games()?;
        let mut dtos = Vec::new();

        for game in games {
            dtos.push(self.game_to_dto(&game)?);
        }

        Ok(dtos)
    }

    pub fn get_game(&self, id: i64) -> Result<Option<GameDto>, String> {
        match self.game_service.get_game(id)? {
            Some(game) => Ok(Some(self.game_to_dto(&game)?)),
            None => Ok(None),
        }
    }

    pub fn get_games_by_genre(&self, genre_id: i64) -> Result<Vec<GameDto>, String> {
        let games = self.game_service.get_games_by_genre(genre_id)?;
        let mut dtos = Vec::new();

        for game in games {
            dtos.push(self.game_to_dto(&game)?);
        }

        Ok(dtos)
    }

    pub fn delete_game(&self, id: i64) -> Result<(), String> {
        self.game_service.delete_game(id)
    }

    pub fn search_games(&self, query: String) -> Result<Vec<GameDto>, String> {
        let all_games = self.get_all_games()?;
        let query_lower = query.to_lowercase();
        
        Ok(all_games
            .into_iter()
            .filter(|game| game.title.to_lowercase().contains(&query_lower))
            .collect())
    }

    fn game_to_dto(&self, game: &Game) -> Result<GameDto, String> {
        let average_rating = self.rating_service
            .get_average_rating(game.id())?
            .unwrap_or(0.0);
        
        let total_ratings = self.rating_service
            .get_total_ratings(game.id())?;

        Ok(GameDto {
            id: game.id(),
            title: game.title().as_str().to_string(),
            description: game.description().clone(),
            genre_id: game.genre_id(),
            release_date: game.release_date().to_string(),
            average_rating,
            total_ratings,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameDto {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub genre_id: i64,
    pub release_date: String,
    pub average_rating: f64,
    pub total_ratings: i64,
}
