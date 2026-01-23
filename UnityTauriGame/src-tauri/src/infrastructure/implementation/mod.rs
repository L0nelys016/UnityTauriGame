pub mod user_repository;
pub mod genre_repository;
pub mod game_repository;
pub mod rating_repository;

pub use user_repository::SQLiteUserRepository;
pub use genre_repository::SQLiteGenreRepository;
pub use game_repository::SQLiteGameRepository;
pub use rating_repository::SQLiteUserRatingRepository;

use rusqlite::{Connection, Result as SqliteResult};
use std::sync::Mutex;

pub struct Database;

impl Database {
    pub fn init(db_path: &str) -> SqliteResult<Mutex<Connection>> {
        let conn = Connection::open(db_path)?;
        
        // Create tables
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS users (
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
                title TEXT NOT NULL UNIQUE,
                genre_id INTEGER NOT NULL,
                rating REAL NOT NULL DEFAULT 0.0,
                FOREIGN KEY(genre_id) REFERENCES genres(id)
            );
            
            CREATE TABLE IF NOT EXISTS user_ratings (
                id INTEGER PRIMARY KEY,
                user_id INTEGER NOT NULL,
                game_id INTEGER NOT NULL,
                score INTEGER NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(user_id, game_id),
                FOREIGN KEY(user_id) REFERENCES users(id),
                FOREIGN KEY(game_id) REFERENCES games(id)
            );",
        )?;
        
        // Insert default genres
        conn.execute_batch(
            "INSERT OR IGNORE INTO genres (id, name) VALUES
                (1, 'Action'),
                (2, 'RPG'),
                (3, 'Strategy'),
                (4, 'Sports'),
                (5, 'Puzzle'),
                (6, 'Adventure'),
                (7, 'Shooter'),
                (8, 'Racing'),
                (9, 'Simulation'),
                (10, 'Indie');",
        )?;
        
        Ok(Mutex::new(conn))
    }
}
