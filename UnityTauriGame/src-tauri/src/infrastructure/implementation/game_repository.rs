use std::sync::{Arc, Mutex};
use rusqlite::{Connection, OptionalExtension};
use crate::domain::{
    abstractions::GameRepository,
    models::{Game, GameTitle},
};

pub struct SQLiteGameRepository {
    connection: Arc<Mutex<Connection>>,
}

impl SQLiteGameRepository {
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        SQLiteGameRepository { connection }
    }
}

impl GameRepository for SQLiteGameRepository {
    fn save(&self, game: &Game) -> Result<(), String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        conn
            .execute(
                "INSERT OR REPLACE INTO games (id, title, description, genre_id, release_date, average_rating, total_ratings) 
                 VALUES (?, ?, ?, ?, ?, ?, ?)",
                rusqlite::params![
                    game.id(),
                    game.title().as_str(),
                    game.description(),
                    game.genre_id(),
                    game.release_date(),
                    game.average_rating().as_f64(),
                    game.total_ratings(),
                ],
            )
            .map_err(|e| format!("Failed to save game: {}", e))?;
        Ok(())
    }

    fn find_by_id(&self, id: i64) -> Result<Option<Game>, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare(
                "SELECT g.id, g.title, g.description, g.release_date, g.average_rating, g.total_ratings, g.genre_id
                 FROM games g WHERE g.id = ?"
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let game = stmt
            .query_row(rusqlite::params![id], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, f64>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, i64>(6)?,
                ))
            })
            .optional()
            .map_err(|e| format!("Failed to query game: {}", e))?;

        if let Some((id, title_str, description, release_date, avg_rating, total_ratings, genre_id)) = game {
            let title = GameTitle::new(title_str)?;
            let mut game = Game::new(id, title, description, genre_id, release_date)?;
            game.update_rating(avg_rating, total_ratings)?;
            Ok(Some(game))
        } else {
            Ok(None)
        }
    }

    fn find_by_title(&self, title: &GameTitle) -> Result<Option<Game>, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare(
                "SELECT g.id, g.title, g.description, g.release_date, g.average_rating, g.total_ratings, g.genre_id
                 FROM games g WHERE g.title = ?"
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let game = stmt
            .query_row(rusqlite::params![title.as_str()], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, f64>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, i64>(6)?,
                ))
            })
            .optional()
            .map_err(|e| format!("Failed to query game: {}", e))?;

        if let Some((id, title_str, description, release_date, avg_rating, total_ratings, genre_id)) = game {
            let title = GameTitle::new(title_str)?;
            let mut game = Game::new(id, title, description, genre_id, release_date)?;
            game.update_rating(avg_rating, total_ratings)?;
            Ok(Some(game))
        } else {
            Ok(None)
        }
    }

    fn find_all(&self) -> Result<Vec<Game>, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare(
                "SELECT g.id, g.title, g.description, g.release_date, g.average_rating, g.total_ratings, g.genre_id
                 FROM games g ORDER BY g.title ASC"
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let games = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, f64>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, i64>(6)?,
                ))
            })
            .map_err(|e| format!("Failed to query games: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect games: {}", e))?;

        let mut result = Vec::new();
        for (id, title_str, description, release_date, avg_rating, total_ratings, genre_id) in games {
            let title = GameTitle::new(title_str)?;
            let mut game = Game::new(id, title, description, genre_id, release_date)?;
            game.update_rating(avg_rating, total_ratings)?;
            result.push(game);
        }
        Ok(result)
    }

    fn find_by_genre(&self, genre_id: i64) -> Result<Vec<Game>, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare(
                "SELECT g.id, g.title, g.description, g.release_date, g.average_rating, g.total_ratings, g.genre_id
                 FROM games g WHERE g.genre_id = ? ORDER BY g.title ASC"
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let games = stmt
            .query_map(rusqlite::params![genre_id], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, f64>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, i64>(6)?,
                ))
            })
            .map_err(|e| format!("Failed to query games: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect games: {}", e))?;

        let mut result = Vec::new();
        for (id, title_str, description, release_date, avg_rating, total_ratings, gen_id) in games {
            let title = GameTitle::new(title_str)?;
            let mut game = Game::new(id, title, description, gen_id, release_date)?;
            game.update_rating(avg_rating, total_ratings)?;
            result.push(game);
        }
        Ok(result)
    }

    fn find_top_rated(&self, limit: i64) -> Result<Vec<Game>, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare(
                "SELECT g.id, g.title, g.description, g.release_date, g.average_rating, g.total_ratings, g.genre_id
                 FROM games g ORDER BY g.average_rating DESC LIMIT ?"
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let games = stmt
            .query_map(rusqlite::params![limit], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, f64>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, i64>(6)?,
                ))
            })
            .map_err(|e| format!("Failed to query games: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect games: {}", e))?;

        let mut result = Vec::new();
        for (id, title_str, description, release_date, avg_rating, total_ratings, genre_id) in games {
            let title = GameTitle::new(title_str)?;
            let mut game = Game::new(id, title, description, genre_id, release_date)?;
            game.update_rating(avg_rating, total_ratings)?;
            result.push(game);
        }
        Ok(result)
    }

    fn delete(&self, id: i64) -> Result<(), String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let rows_affected = conn
            .execute("DELETE FROM games WHERE id = ?", rusqlite::params![id])
            .map_err(|e| format!("Failed to delete game: {}", e))?;

        if rows_affected == 0 {
            return Err("Game not found".to_string());
        }
        Ok(())
    }

    fn count(&self) -> Result<i64, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare("SELECT COUNT(*) FROM games")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let count = stmt
            .query_row([], |row| row.get(0))
            .map_err(|e| format!("Failed to count games: {}", e))?;

        Ok(count)
    }
}
