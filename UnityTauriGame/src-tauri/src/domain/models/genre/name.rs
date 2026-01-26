#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenreName(String);

impl GenreName {
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err("Название жанра не может быть пустым".to_string());
        }
        if value.len() < 2 {
            return Err("Название жанра должно содержать минимум 2 символа".to_string());
        }
        if value.len() > 50 {
            return Err("Название жанра не должно превышать 50 символов".to_string());
        }
        Ok(GenreName(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}
