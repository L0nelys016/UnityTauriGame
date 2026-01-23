use super::entity::User;
use super::username::Username;

pub trait UserRepository {
    fn save(&self, user: &User) -> Result<(), String>;
    fn find_by_id(&self, id: i64) -> Result<Option<User>, String>;
    fn find_by_username(&self, username: &Username) -> Result<Option<User>, String>;
    fn find_all(&self) -> Result<Vec<User>, String>;
    fn delete(&self, id: i64) -> Result<(), String>;
    fn exists_by_username(&self, username: &Username) -> Result<bool, String>;
}
