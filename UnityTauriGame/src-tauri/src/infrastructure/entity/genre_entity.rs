#[derive(Debug, Clone)]
pub struct GenreEntity {
    pub id: i64,
    pub name: String,
}

impl GenreEntity {
    pub fn new(id: i64, name: String) -> Self {
        GenreEntity { id, name }
    }
}
