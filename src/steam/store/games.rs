use crate::steam::store::SteamStoreError;
use crate::steam::Game;
use crate::utils::filter::StringFilter;
use async_graphql::{InputObject, SimpleObject};
use std::time::{SystemTime,  UNIX_EPOCH};

#[derive(Debug, Clone, SimpleObject, InputObject)]
pub struct GameFilter {
    pub name: Option<StringFilter>,
}

pub struct SteamGamesStore {
    pub pool: sqlx::Pool<sqlx::Sqlite>,
}
impl SteamGamesStore {
    pub async fn upsert(&self, game: &Game) -> Result<(), SteamStoreError> {
        let at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();

        sqlx::query!(
            r#"
            INSERT INTO steam_games(id, name, upserted_at)
            VALUES(?1, ?2, ?3)
            ON CONFLICT(id)
            DO UPDATE SET name=excluded.name
              , upserted_at=?3
            "#,
            game.id,
            game.name,
            at,
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(SteamStoreError::Sqlx)
    }

    pub async fn get_last_upserted(&self) -> Result<Option<Game>, SteamStoreError> {
        sqlx::query_as!(
            Game,
            r#"
            SELECT id as "id!: _", name as "name!"
            FROM steam_games
            ORDER BY upserted_at DESC
            LIMIT 1
            "#
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(SteamStoreError::Sqlx)
    }

    pub async fn get_by_id(&self, id: u32) -> Result<Option<Game>, SteamStoreError> {
        sqlx::query_as!(
            Game,
            r#"
            SELECT id as "id!: _", name as "name!"
            FROM steam_games
            WHERE id = ?
            LIMIT 1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(SteamStoreError::Sqlx)
    }

    pub async fn get_all(
        &self,
        filter: Option<GameFilter>,
        first: usize,
        after: Option<u32>,
    ) -> Result<Vec<Game>, SteamStoreError> {
        let filter = filter
            .and_then(|filter| filter.name)
            .and_then(|name| name.contains)
            .map(|contains| format!("%{}%", contains.to_lowercase()));

        let first = first as u32;

        sqlx::query_as!(
            Game,
            r#"
            SELECT id as "id!: _", name as "name!"
            FROM steam_games
            WHERE CASE WHEN ?1 IS NULL THEN TRUE ELSE name LIKE ?1 END
              AND CASE WHEN ?2 IS NULL THEN TRUE ELSE id > ?2 END
            ORDER BY id ASC
            LIMIT ?3
            "#,
            filter,
            after,
            first
        )
        .fetch_all(&self.pool)
        .await
        .map_err(SteamStoreError::Sqlx)
    }
}
