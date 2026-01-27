use std::sync::Arc;
use crate::application::UserService;

pub struct AuthUseCase {
    user_service: Arc<UserService>,
}

impl AuthUseCase {
    pub fn new(user_service: Arc<UserService>) -> Self {
        AuthUseCase { user_service }
    }

    pub fn login(&self, username: String, password: String) -> Result<UserDto, String> {
        let user = self.user_service
            .get_user_by_username(username.clone())
            .map_err(|e| format!("Не удалось найти пользователя: {}", e))?
            .ok_or_else(|| "Неверное имя пользователя или пароль".to_string())?;

        // Simple password comparison (in production, use hashing)
        if user.password().as_str() != password {
            return Err("Неверное имя пользователя или пароль".to_string());
        }

        Ok(UserDto {
            id: user.id(),
            username: user.username().as_str().to_string(),
            role: user.role().as_i32(),
        })
    }

    pub fn get_user_by_id(&self, id: i64) -> Result<Option<UserDto>, String> {
        match self.user_service.get_user(id)? {
            Some(user) => Ok(Some(UserDto {
                id: user.id(),
                username: user.username().as_str().to_string(),
                role: user.role().as_i32(),
            })),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserDto {
    pub id: i64,
    pub username: String,
    pub role: i32,
}
