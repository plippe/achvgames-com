pub mod api_steampowered_com;
pub mod environment;
pub mod graphql;
pub mod store;
pub mod worker;

use async_graphql::SimpleObject;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, SimpleObject)]
#[graphql(complex)]
pub struct Game {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct GameAchievement {
    pub name: String,
    pub description: Option<String>,
    pub icon_locked_url: String,
    pub icon_unlocked_url: String,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct GameImages {
    pub header_url: String,
}

#[derive(Debug, Clone)]
pub struct GameWithAchievements {
    pub game: Game,
    pub achievements: Vec<GameAchievement>,
}
