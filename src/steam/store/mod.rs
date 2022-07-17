pub mod game_achievements;
pub mod games;

#[derive(Debug)]
pub enum SteamStoreError {
    Sqlx(sqlx::Error),
}
