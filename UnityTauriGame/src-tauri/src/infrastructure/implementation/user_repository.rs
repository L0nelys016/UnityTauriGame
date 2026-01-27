use std::sync::{Arc, Mutex};
use rusqlite::{Connection, OptionalExtension};
use crate::domain::{
    abstractions::UserRepository,
    models::{User, UserRole, Username, Password},
};

pub struct SQLiteUserRepository {
    connection: Arc<Mutex<Connection>>,
}

impl SQLiteUserRepository {
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        SQLiteUserRepository { connection }
    }
}

impl UserRepository for SQLiteUserRepository {
    fn save(&self, user: &User) -> Result<(), String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        conn
            .execute(
                "INSERT OR REPLACE INTO users (id, username, password, role) VALUES (?, ?, ?, ?)",
                rusqlite::params![
                    user.id(),
                    user.username().as_str(),
                    user.password().as_str(),
                    user.role().as_i32(),
                ],
            )
            .map_err(|e| format!("Failed to save user: {}", e))?;
        Ok(())
    }

    fn find_by_id(&self, id: i64) -> Result<Option<User>, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare("SELECT id, username, password, role FROM users WHERE id = ?")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let user = stmt
            .query_row(rusqlite::params![id], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i32>(3)?,
                ))
            })
            .optional()
            .map_err(|e| format!("Failed to query user: {}", e))?;

        if let Some((id, username_str, password_str, role_i32)) = user {
            let username = Username::new(username_str)?;
            let password = Password::new(password_str)?;
            let role = UserRole::from_i32(role_i32)?;
            Ok(Some(User::new(id, username, password, role)))
        } else {
            Ok(None)
        }
    }

    fn find_by_username(&self, username: &Username) -> Result<Option<User>, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare("SELECT id, username, password, role FROM users WHERE username = ?")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let user = stmt
            .query_row(rusqlite::params![username.as_str()], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i32>(3)?,
                ))
            })
            .optional()
            .map_err(|e| format!("Failed to query user: {}", e))?;

        if let Some((id, username_str, password_str, role_i32)) = user {
            let username = Username::new(username_str)?;
            let password = Password::new(password_str)?;
            let role = UserRole::from_i32(role_i32)?;
            Ok(Some(User::new(id, username, password, role)))
        } else {
            Ok(None)
        }
    }

    fn find_all(&self) -> Result<Vec<User>, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare("SELECT id, username, password, role FROM users")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let users = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i32>(3)?,
                ))
            })
            .map_err(|e| format!("Failed to query users: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect users: {}", e))?;

        let mut result = Vec::new();
        for (id, username_str, password_str, role_i32) in users {
            let username = Username::new(username_str)?;
            let password = Password::new(password_str)?;
            let role = UserRole::from_i32(role_i32)?;
            result.push(User::new(id, username, password, role));
        }
        Ok(result)
    }

    fn delete(&self, id: i64) -> Result<(), String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let rows_affected = conn
            .execute("DELETE FROM users WHERE id = ?", rusqlite::params![id])
            .map_err(|e| format!("Failed to delete user: {}", e))?;

        if rows_affected == 0 {
            return Err("Пользователь не найден".to_string());
        }
        Ok(())
    }

    fn exists_by_username(&self, username: &Username) -> Result<bool, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| "Database connection poisoned".to_string())?;

        let mut stmt = conn
            .prepare("SELECT 1 FROM users WHERE username = ?")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let exists = stmt
            .exists(rusqlite::params![username.as_str()])
            .map_err(|e| format!("Failed to check username existence: {}", e))?;

        Ok(exists)
    }
}
