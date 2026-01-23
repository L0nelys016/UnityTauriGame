use super::role::UserRole;
use super::username::Username;
use super::password::Password;

#[derive(Debug, Clone)]
pub struct User {
    id: i64,
    username: Username,
    password: Password,
    role: UserRole,
}

impl User {
    pub fn new(id: i64, username: Username, password: Password, role: UserRole) -> Self {
        User {
            id,
            username,
            password,
            role,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn password(&self) -> &Password {
        &self.password
    }

    pub fn role(&self) -> UserRole {
        self.role
    }

    pub fn change_password(&mut self, password: Password) {
        self.password = password;
    }

    pub fn change_role(&mut self, role: UserRole) {
        self.role = role;
    }
}
