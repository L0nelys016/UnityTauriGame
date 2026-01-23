use std::sync::Arc;
use crate::application::{UserService, GenreService, GameService, RatingService};

pub struct PresentationLayer {
    pub user_service: Arc<UserService>,
    pub genre_service: Arc<GenreService>,
    pub game_service: Arc<GameService>,
    pub rating_service: Arc<RatingService>,
}

impl PresentationLayer {
    pub fn new(
        user_service: Arc<UserService>,
        genre_service: Arc<GenreService>,
        game_service: Arc<GameService>,
        rating_service: Arc<RatingService>,
    ) -> Self {
        PresentationLayer {
            user_service,
            genre_service,
            game_service,
            rating_service,
        }
    }
}
