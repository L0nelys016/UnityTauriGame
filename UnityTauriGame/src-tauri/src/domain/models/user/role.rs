#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum UserRole {
    User = 0,
    Admin = 1,
}

impl UserRole {
    pub fn from_i32(value: i32) -> Result<Self, String> {
        match value {
            0 => Ok(UserRole::User),
            1 => Ok(UserRole::Admin),
            _ => Err(format!("Неверное значение роли: {}", value)),
        }
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}
