pub mod user_repository;
pub mod genre_repository;
pub mod game_repository;
pub mod rating_repository;

pub use user_repository::SQLiteUserRepository;
pub use genre_repository::SQLiteGenreRepository;
pub use game_repository::SQLiteGameRepository;
pub use rating_repository::SQLiteUserRatingRepository;

use rusqlite::{Connection, Result as SqliteResult};
use std::sync::{Arc, Mutex};

pub struct Database;

impl Database {
    pub fn init(db_path: &str) -> SqliteResult<Arc<Mutex<Connection>>> {
        let conn = Connection::open(db_path)?;
        
        // Create tables
        conn.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                role INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS genres (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            );

            CREATE TABLE IF NOT EXISTS games (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                genre_id INTEGER NOT NULL,
                release_date TEXT NOT NULL,
                average_rating REAL DEFAULT 0,
                total_ratings INTEGER DEFAULT 0,
                FOREIGN KEY (genre_id) REFERENCES genres(id)
                    ON DELETE RESTRICT
                    ON UPDATE CASCADE
            );

            CREATE TABLE IF NOT EXISTS user_ratings (
                id INTEGER PRIMARY KEY,
                user_id INTEGER NOT NULL,
                game_id INTEGER NOT NULL,
                rating INTEGER NOT NULL CHECK (rating BETWEEN 1 AND 5),
                created_at TEXT NOT NULL,
                UNIQUE(user_id, game_id),
                FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
                FOREIGN KEY(game_id) REFERENCES games(id) ON DELETE CASCADE
            );
            "#,
        )?;
        
        // Insert default genres
        conn.execute_batch(
            r#"
            INSERT OR IGNORE INTO genres (id, name) VALUES
                (1, 'Action'),
                (2, 'RPG'),
                (3, 'Strategy'),
                (4, 'Sports'),
                (5, 'Puzzle'),
                (6, 'Adventure'),
                (7, 'Shooter'),
                (8, 'Racing'),
                (9, 'Simulation'),
                (10, 'Indie');

            INSERT OR IGNORE INTO users(id, username, password, role) VALUES
                (1, 'admin', 'admin', 1),
                (2, 'user', 'user', 0);
            "#
        )?;
        
        Ok(Arc::new(Mutex::new(conn)))
    }
}
