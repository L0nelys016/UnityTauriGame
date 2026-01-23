#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenreName(String);

impl GenreName {
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err("Genre name cannot be empty".to_string());
        }
        if value.len() < 2 {
            return Err("Genre name must be at least 2 characters".to_string());
        }
        if value.len() > 50 {
            return Err("Genre name must not exceed 50 characters".to_string());
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
