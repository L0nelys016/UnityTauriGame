pub mod connection;
pub mod migration;

use rusqlite::connection;

pub fn init_database(db_path: &str) -> Connection {
    let conn = connection::establish_connection(db_path);
    migration::run_migration(&conn);
    conn
}