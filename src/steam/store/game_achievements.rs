use crate::steam::store::SteamStoreError;
use crate::steam::GameAchievement;
use chaining::Pipe;
use futures::stream::{self, StreamExt, TryStreamExt};
use sqlx::postgres::PgPool;

pub struct SteamGameAchievementsStore {
    pub pool: PgPool,
}

impl SteamGameAchievementsStore {
    pub async fn upsert_bulk(
        &self,
        game_id: i64,
        game_achievements: &[GameAchievement],
    ) -> Result<(), SteamStoreError> {
        game_achievements
            .pipe(stream::iter)
            .then(|achv| self.upsert(game_id, achv))
            .try_collect()
            .await
    }

    async fn upsert(
        &self,
        game_id: i64,
        game_achievement: &GameAchievement,
    ) -> Result<(), SteamStoreError> {
        sqlx::query!(
            r#"
            INSERT INTO steam_game_achievements(steam_game_id, name, description, icon_locked_url, icon_unlocked_url)
            VALUES($1, $2, $3, $4, $5)
            ON CONFLICT(steam_game_id, name)
            DO UPDATE SET description=excluded.description
              , icon_locked_url=excluded.icon_locked_url
              , icon_unlocked_url=excluded.icon_unlocked_url
            "#,
            game_id,
            game_achievement.name,
            game_achievement.description,
            game_achievement.icon_locked_url,
            game_achievement.icon_unlocked_url,
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(SteamStoreError::Sqlx)
    }

    pub async fn get_by_game_id(
        &self,
        game_id: i64,
    ) -> Result<Vec<GameAchievement>, SteamStoreError> {
        sqlx::query_as!(
            GameAchievement,
            r#"
            SELECT name, description, icon_locked_url, icon_unlocked_url
            FROM steam_game_achievements
            WHERE steam_game_id = $1
            "#,
            game_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(SteamStoreError::Sqlx)
    }
}
