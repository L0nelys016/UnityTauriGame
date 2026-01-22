use crate::dto::GenreRow;

#[derive(Debug)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

impl From<GenreRow> for Genre {
    fn from(row: GenreRow) -> Self {
        Genre {
            id: row.id,
            name: row.name,
        }
    }
}
