use std::sync::Arc;
use crate::domain::{
    abstractions::GenreRepository,
    models::{Genre, GenreName},
};

pub struct GenreService {
    repository: Arc<dyn GenreRepository>,
}

impl GenreService {
    pub fn new(repository: Arc<dyn GenreRepository>) -> Self {
        GenreService { repository }
    }

    pub fn create_genre(&self, id: i64, name: String) -> Result<Genre, String> {
        let genre_name = GenreName::new(name)?;

        let genre = Genre::new(id, genre_name);
        self.repository.save(&genre)?;
        Ok(genre)
    }

    pub fn get_genre(&self, id: i64) -> Result<Option<Genre>, String> {
        self.repository.find_by_id(id)
    }

    pub fn get_genre_by_name(&self, name: String) -> Result<Option<Genre>, String> {
        let genre_name = GenreName::new(name)?;
        self.repository.find_by_name(&genre_name)
    }

    pub fn get_all_genres(&self) -> Result<Vec<Genre>, String> {
        self.repository.find_all()
    }

    pub fn delete_genre(&self, id: i64) -> Result<(), String> {
        self.repository.delete(id)
    }

    pub fn count_genres(&self) -> Result<i64, String> {
        self.repository.count()
    }
}
