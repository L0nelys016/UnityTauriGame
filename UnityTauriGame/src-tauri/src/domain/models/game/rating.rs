#[derive(Debug, Clone, PartialEq)]
pub struct Rating(f64);

impl Rating {
    pub fn new(value: f64) -> Result<Self, String> {
        if value < 0.0 || value > 5.0 {
            return Err("Рейтинг должен быть от 0 до 5".to_string());
        }
        Ok(Rating(value))
    }

    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl Default for Rating {
    fn default() -> Self {
        Rating(0.0)
    }
}
