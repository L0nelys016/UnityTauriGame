use rusqlite::Connection;

pub fn establish_connection(path: &str) -> Connection {
    Connection::open(path).expect("Failed to open SQLite database")
}