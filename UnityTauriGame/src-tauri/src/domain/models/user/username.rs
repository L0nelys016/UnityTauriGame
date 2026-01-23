#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username(String);

impl Username {
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        if value.len() < 3 {
            return Err("Username must be at least 3 characters".to_string());
        }
        if value.len() > 50 {
            return Err("Username must not exceed 50 characters".to_string());
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
