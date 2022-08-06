use crate::steam::store::SteamStoreError;
use crate::steam::Game;

pub struct SteamGamesStore {
    pub pool: sqlx::Pool<sqlx::Sqlite>,
}
impl SteamGamesStore {
    pub async fn upsert(&self, game: &Game) -> Result<(), SteamStoreError> {
        sqlx::query!(
            r#"
            INSERT INTO steam_games(id, name)
            VALUES(?, ?)
            ON CONFLICT(id) DO UPDATE SET name=excluded.name
            "#,
            game.id,
            game.name
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(SteamStoreError::Sqlx)
    }

    pub async fn get_last_updated(&self) -> Result<Option<Game>, SteamStoreError> {
        sqlx::query_as!(
            Game,
            r#"
            SELECT id as "id!: _", name as "name!"
            FROM steam_games
            ORDER BY updated_at DESC
            LIMIT 1
            "#
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(SteamStoreError::Sqlx)
    }

    pub async fn get(
        &self,
        first: usize,
        after: Option<u32>,
    ) -> Result<Vec<Game>, SteamStoreError> {
        let first = first as u32;
        let after = after.unwrap_or_default();

        sqlx::query_as!(
            Game,
            r#"
            SELECT id as "id!: _", name as "name!"
            FROM steam_games
            WHERE id > ?
            ORDER BY id ASC
            LIMIT ?
            "#,
            after,
            first
        )
        .fetch_all(&self.pool)
        .await
        .map_err(SteamStoreError::Sqlx)
    }
}
