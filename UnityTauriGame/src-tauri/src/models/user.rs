use std::convert::TryFrom;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRole {
    User = 0,
    Admin = 1,
}

impl TryFrom<i32> for UserRole {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(UserRole::User),
            1 => Ok(UserRole::Admin),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub role: UserRole,
}
