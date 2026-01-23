use super::name::GenreName;

#[derive(Debug, Clone)]
pub struct Genre {
    id: i64,
    name: GenreName,
}

impl Genre {
    pub fn new(id: i64, name: GenreName) -> Self {
        Genre { id, name }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> &GenreName {
        &self.name
    }

    pub fn rename(&mut self, name: GenreName) {
        self.name = name;
    }
}
