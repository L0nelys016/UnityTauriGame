#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err("Пароль не может быть пустым".to_string());
        }
        if value.len() < 4 {
            return Err("Пароль должен содержать минимум 4 символа".to_string());
        }
        Ok(Password(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}
