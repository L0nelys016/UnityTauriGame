#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub role: i32,
}

impl UserEntity {
    pub fn new(id: i64, username: String, password: String, role: i32) -> Self {
        UserEntity {
            id,
            username,
            password,
            role,
        }
    }
}
