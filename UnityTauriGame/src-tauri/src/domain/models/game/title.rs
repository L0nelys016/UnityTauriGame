#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameTitle(String);

impl GameTitle {
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err("Game title cannot be empty".to_string());
        }
        if value.len() < 2 {
            return Err("Game title must be at least 2 characters".to_string());
        }
        if value.len() > 100 {
            return Err("Game title must not exceed 100 characters".to_string());
        }
        Ok(GameTitle(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
