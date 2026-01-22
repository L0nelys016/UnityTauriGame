use std::convert::TryFrom;
use crate::dto::UserRow;

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

impl TryFrom<UserRow> for User {
    type Error = ();

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        Ok(User {
            id: row.id,
            username: row.username,
            password: row.password,
            role: UserRole::try_from(row.role)?,
        })
    }
}