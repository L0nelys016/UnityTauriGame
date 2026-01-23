use std::sync::Arc;
use rusqlite::{Connection, OptionalExtension};
use crate::domain::{
    abstractions::GenreRepository,
    models::{Genre, GenreName},
};

pub struct SQLiteGenreRepository {
    connection: Arc<Connection>,
}

impl SQLiteGenreRepository {
    pub fn new(connection: Arc<Connection>) -> Self {
        SQLiteGenreRepository { connection }
    }
}

impl GenreRepository for SQLiteGenreRepository {
    fn save(&self, genre: &Genre) -> Result<(), String> {
        self.connection
            .execute(
                "INSERT OR REPLACE INTO genres (id, name) VALUES (?, ?)",
                rusqlite::params![genre.id(), genre.name().as_str()],
            )
            .map_err(|e| format!("Failed to save genre: {}", e))?;
        Ok(())
    }

    fn find_by_id(&self, id: i64) -> Result<Option<Genre>, String> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, name FROM genres WHERE id = ?")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let genre = stmt
            .query_row(rusqlite::params![id], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
            })
            .optional()
            .map_err(|e| format!("Failed to query genre: {}", e))?;

        if let Some((id, name_str)) = genre {
            let name = GenreName::new(name_str)?;
            Ok(Some(Genre::new(id, name)))
        } else {
            Ok(None)
        }
    }

    fn find_by_name(&self, name: &GenreName) -> Result<Option<Genre>, String> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, name FROM genres WHERE name = ?")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let genre = stmt
            .query_row(rusqlite::params![name.as_str()], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
            })
            .optional()
            .map_err(|e| format!("Failed to query genre: {}", e))?;

        if let Some((id, name_str)) = genre {
            let name = GenreName::new(name_str)?;
            Ok(Some(Genre::new(id, name)))
        } else {
            Ok(None)
        }
    }

    fn find_all(&self) -> Result<Vec<Genre>, String> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, name FROM genres ORDER BY name ASC")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let genres = stmt
            .query_map([], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|e| format!("Failed to query genres: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect genres: {}", e))?;

        let mut result = Vec::new();
        for (id, name_str) in genres {
            let name = GenreName::new(name_str)?;
            result.push(Genre::new(id, name));
        }
        Ok(result)
    }

    fn delete(&self, id: i64) -> Result<(), String> {
        let rows_affected = self
            .connection
            .execute("DELETE FROM genres WHERE id = ?", rusqlite::params![id])
            .map_err(|e| format!("Failed to delete genre: {}", e))?;

        if rows_affected == 0 {
            return Err("Genre not found".to_string());
        }
        Ok(())
    }

    fn count(&self) -> Result<i64, String> {
        let mut stmt = self
            .connection
            .prepare("SELECT COUNT(*) FROM genres")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let count = stmt
            .query_row([], |row| row.get(0))
            .map_err(|e| format!("Failed to count genres: {}", e))?;

        Ok(count)
    }
}
