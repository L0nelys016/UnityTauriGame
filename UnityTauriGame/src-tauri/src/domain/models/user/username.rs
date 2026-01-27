#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username(String);

impl Username {
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err("Имя пользователя не может быть пустым".to_string());
        }
        if value.len() < 3 {
            return Err("Имя пользователя должно содержать минимум 3 символа".to_string());
        }
        if value.len() > 50 {
            return Err("Имя пользователя не должно превышать 50 символов".to_string());
        }
        Ok(Username(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
