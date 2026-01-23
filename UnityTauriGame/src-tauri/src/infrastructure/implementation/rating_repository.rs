use std::sync::{Arc, Mutex};
use rusqlite::{Connection, OptionalExtension};
use crate::domain::{
    abstractions::UserRatingRepository,
    models::{UserRating, RatingScore},
};

pub struct SQLiteUserRatingRepository {
    connection: Arc<Mutex<Connection>>,
}

impl SQLiteUserRatingRepository {
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        SQLiteUserRatingRepository { connection }
    }
}

impl UserRatingRepository for SQLiteUserRatingRepository {
    fn save(&self, rating: &UserRating) -> Result<(), String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        conn
            .execute(
                "INSERT OR REPLACE INTO user_ratings (id, user_id, game_id, rating, created_at) 
                 VALUES (?, ?, ?, ?, ?)",
                rusqlite::params![
                    rating.id(),
                    rating.user_id(),
                    rating.game_id(),
                    rating.score().as_i32(),
                    rating.created_at(),
                ],
            )
            .map_err(|e| format!("Failed to save rating: {}", e))?;
        Ok(())
    }

    fn find_by_id(&self, id: i64) -> Result<Option<UserRating>, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare("SELECT id, user_id, game_id, rating, created_at FROM user_ratings WHERE id = ?")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let rating = stmt
            .query_row(rusqlite::params![id], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, i32>(3)?,
                    row.get::<_, String>(4)?,
                ))
            })
            .optional()
            .map_err(|e| format!("Failed to query rating: {}", e))?;

        if let Some((id, user_id, game_id, score_i32, created_at)) = rating {
            let score = RatingScore::new(score_i32)?;
            Ok(Some(UserRating::new(id, user_id, game_id, score, created_at)))
        } else {
            Ok(None)
        }
    }

    fn find_by_user_and_game(&self, user_id: i64, game_id: i64) -> Result<Option<UserRating>, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare("SELECT id, user_id, game_id, rating, created_at FROM user_ratings WHERE user_id = ? AND game_id = ?")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let rating = stmt
            .query_row(rusqlite::params![user_id, game_id], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, i32>(3)?,
                    row.get::<_, String>(4)?,
                ))
            })
            .optional()
            .map_err(|e| format!("Failed to query rating: {}", e))?;

        if let Some((id, user_id, game_id, score_i32, created_at)) = rating {
            let score = RatingScore::new(score_i32)?;
            Ok(Some(UserRating::new(id, user_id, game_id, score, created_at)))
        } else {
            Ok(None)
        }
    }

    fn find_by_game(&self, game_id: i64) -> Result<Vec<UserRating>, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare("SELECT id, user_id, game_id, rating, created_at FROM user_ratings WHERE game_id = ? ORDER BY created_at DESC")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let ratings = stmt
            .query_map(rusqlite::params![game_id], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, i32>(3)?,
                    row.get::<_, String>(4)?,
                ))
            })
            .map_err(|e| format!("Failed to query ratings: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect ratings: {}", e))?;

        let mut result = Vec::new();
        for (id, user_id, game_id, score_i32, created_at) in ratings {
            let score = RatingScore::new(score_i32)?;
            result.push(UserRating::new(id, user_id, game_id, score, created_at));
        }
        Ok(result)
    }

    fn find_by_user(&self, user_id: i64) -> Result<Vec<UserRating>, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare("SELECT id, user_id, game_id, rating, created_at FROM user_ratings WHERE user_id = ? ORDER BY created_at DESC")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let ratings = stmt
            .query_map(rusqlite::params![user_id], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, i32>(3)?,
                    row.get::<_, String>(4)?,
                ))
            })
            .map_err(|e| format!("Failed to query ratings: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect ratings: {}", e))?;

        let mut result = Vec::new();
        for (id, user_id, game_id, score_i32, created_at) in ratings {
            let score = RatingScore::new(score_i32)?;
            result.push(UserRating::new(id, user_id, game_id, score, created_at));
        }
        Ok(result)
    }

    fn delete(&self, id: i64) -> Result<(), String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let rows_affected = conn
            .execute("DELETE FROM user_ratings WHERE id = ?", rusqlite::params![id])
            .map_err(|e| format!("Failed to delete rating: {}", e))?;

        if rows_affected == 0 {
            return Err("Rating not found".to_string());
        }
        Ok(())
    }

    fn delete_by_user_and_game(&self, user_id: i64, game_id: i64) -> Result<(), String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let rows_affected = conn
            .execute(
                "DELETE FROM user_ratings WHERE user_id = ? AND game_id = ?",
                rusqlite::params![user_id, game_id],
            )
            .map_err(|e| format!("Failed to delete rating: {}", e))?;

        if rows_affected == 0 {
            return Err("Rating not found".to_string());
        }
        Ok(())
    }

    fn get_average_rating(&self, game_id: i64) -> Result<Option<f64>, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare("SELECT AVG(rating) FROM user_ratings WHERE game_id = ?")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let average = stmt
            .query_row(rusqlite::params![game_id], |row| row.get(0))
            .map_err(|e| format!("Failed to query average rating: {}", e))?;

        Ok(average)
    }

    fn get_total_ratings(&self, game_id: i64) -> Result<i64, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare("SELECT COUNT(*) FROM user_ratings WHERE game_id = ?")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let count = stmt
            .query_row(rusqlite::params![game_id], |row| row.get(0))
            .map_err(|e| format!("Failed to count ratings: {}", e))?;

        Ok(count)
    }
}
