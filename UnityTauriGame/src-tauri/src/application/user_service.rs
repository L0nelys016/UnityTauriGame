use std::sync::Arc;
use crate::domain::{
    abstractions::UserRepository,
    models::{User, UserRole, Username, Password},
};

pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        UserService { repository }
    }

    pub fn create_user(
        &self,
        id: i64,
        username: String,
        password: String,
        role: UserRole,
    ) -> Result<User, String> {
        let username = Username::new(username)?;
        let password = Password::new(password)?;

        // Check if user already exists
        if self.repository.exists_by_username(&username)? {
            return Err("Пользователь с таким именем уже существует".to_string());
        }

        let user = User::new(id, username, password, role);
        self.repository.save(&user)?;
        Ok(user)
    }

    pub fn get_user(&self, id: i64) -> Result<Option<User>, String> {
        self.repository.find_by_id(id)
    }

    pub fn get_user_by_username(&self, username: String) -> Result<Option<User>, String> {
        let username = Username::new(username)?;
        self.repository.find_by_username(&username)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, String> {
        self.repository.find_all()
    }

    pub fn delete_user(&self, id: i64) -> Result<(), String> {
        self.repository.delete(id)
    }
}
