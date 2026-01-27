#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RatingScore(i32);

impl RatingScore {
    pub fn new(value: i32) -> Result<Self, String> {
        if value < 1 || value > 5 {
            return Err("Оценка должна быть от 1 до 5".to_string());
        }
        Ok(RatingScore(value))
    }

    pub fn as_i32(&self) -> i32 {
        self.0
    }
}
