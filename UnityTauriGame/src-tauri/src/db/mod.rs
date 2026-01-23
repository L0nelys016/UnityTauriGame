pub mod connection;
pub mod migrations;

pub use connection::*;
pub use migrations::*;

use rusqlite::Connection;

pub fn init_database(db_path: &str) -> Connection {
    let conn = connection::establish_connection(db_path);
    migrations::run_migrations(&conn);
    conn
}
