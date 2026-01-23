use super::entity::Genre;
use super::genre_name::GenreName;

pub trait GenreRepository {
    fn save(&self, genre: &Genre) -> Result<(), String>;
    fn find_by_id(&self, id: i64) -> Result<Option<Genre>, String>;
    fn find_by_name(&self, name: &GenreName) -> Result<Option<Genre>, String>;
    fn find_all(&self) -> Result<Vec<Genre>, String>;
    fn delete(&self, id: i64) -> Result<(), String>;
    fn count(&self) -> Result<i64, String>;
}
