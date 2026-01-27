#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameTitle(String);

impl GameTitle {
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err("Название игры не может быть пустым".to_string());
        }
        if value.len() < 2 {
            return Err("Название игры должно содержать минимум 2 символа".to_string());
        }
        if value.len() > 100 {
            return Err("Название игры не должно превышать 100 символов".to_string());
        }
        Ok(GameTitle(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
