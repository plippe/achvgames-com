use achvgames_com::environment;
use achvgames_com::steam::api_steampowered_com::SteamClient;
use achvgames_com::steam::store::game_achievements::SteamGameAchievementsStore;
use achvgames_com::steam::store::games::SteamGamesStore;
use achvgames_com::steam::worker::SteamWorker;
use sqlx::sqlite::SqlitePool;
use surf::Client as SurfClient;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Hello, world!");

    let pool = SqlitePool::connect(&environment::DATABASE_URL).await?;

    let steam_api_client = SteamClient {
        http_client: SurfClient::default(),
    };

    let steam_game_store = SteamGamesStore { pool: pool.clone() };
    let steam_game_achievement_store = SteamGameAchievementsStore { pool: pool.clone() };

    let worker = SteamWorker {
        api_client: steam_api_client,
        games_store: steam_game_store,
        game_achievements_store: steam_game_achievement_store,
    };

    worker.work().await?;

    Ok(())
}
