use std::sync::Arc;
use crate::application::GenreService;

pub struct GenreUseCase {
    genre_service: Arc<GenreService>,
}

impl GenreUseCase {
    pub fn new(genre_service: Arc<GenreService>) -> Self {
        GenreUseCase { genre_service }
    }

    pub fn get_all_genres(&self) -> Result<Vec<GenreDto>, String> {
        let genres = self.genre_service.get_all_genres()?;
        Ok(genres
            .into_iter()
            .map(|g| GenreDto {
                id: g.id(),
                name: g.name().as_str().to_string(),
            })
            .collect())
    }

    pub fn get_genre(&self, id: i64) -> Result<Option<GenreDto>, String> {
        match self.genre_service.get_genre(id)? {
            Some(genre) => Ok(Some(GenreDto {
                id: genre.id(),
                name: genre.name().as_str().to_string(),
            })),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenreDto {
    pub id: i64,
    pub name: String,
}
