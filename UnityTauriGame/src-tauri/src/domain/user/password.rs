#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err("Password cannot be empty".to_string());
        }
        if value.len() < 6 {
            return Err("Password must be at least 6 characters".to_string());
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
