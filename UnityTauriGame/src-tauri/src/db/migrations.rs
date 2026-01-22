use rusqlite::Connection;

pub fn run_migrations(conn: &Connection) {
    conn.execute_batch(
        r#"
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            role INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS genres (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS games (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
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
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            game_id INTEGER NOT NULL,
            rating INTEGER NOT NULL CHECK (rating BETWEEN 1 AND 5),
            created_at TEXT NOT NULL,

            UNIQUE(user_id, game_id),
            FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY(game_id) REFERENCES games(id) ON DELETE CASCADE
        );

        -- SEED GENRES
        INSERT OR IGNORE INTO genres (name) VALUES
            ('Action packed'),
            ('Adventures'),
            ('Strategies'),
            ('Role-playing games'),
            ('Races'),
            ('Simulators');
        "#,
    )
    .expect("Failed to run database migrations");
}
