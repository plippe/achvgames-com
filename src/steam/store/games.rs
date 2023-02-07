use crate::steam::store::SteamStoreError;
use crate::steam::Game;
use crate::utils::filter::StringFilter;
use async_graphql::{InputObject, SimpleObject};
use sqlx::postgres::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, SimpleObject, InputObject)]
pub struct GameFilter {
    pub name: Option<StringFilter>,
}

pub struct SteamGamesStore {
    pub pool: PgPool,
}
impl SteamGamesStore {
    pub async fn upsert(&self, game: &Game) -> Result<(), SteamStoreError> {
        let at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        sqlx::query!(
            r#"
            INSERT INTO steam_games(id, name, upserted_at)
            VALUES($1, $2, $3)
            ON CONFLICT(id)
            DO UPDATE SET name=excluded.name
              , upserted_at=$3
            "#,
            game.id,
            game.name,
            at as i64,
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

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Game>, SteamStoreError> {
        sqlx::query_as!(
            Game,
            r#"
            SELECT id as "id!: _", name as "name!"
            FROM steam_games
            WHERE id = $1
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
        first: u8,
        after: Option<i64>,
    ) -> Result<Vec<Game>, SteamStoreError> {
        let filter = filter
            .and_then(|filter| filter.name)
            .and_then(|name| name.contains)
            .map(|contains| format!("%{}%", contains.to_lowercase()));

        sqlx::query_as!(
            Game,
            r#"
            SELECT id as "id!: _", name as "name!"
            FROM steam_games
            WHERE CASE WHEN $1::VARCHAR IS NULL THEN TRUE ELSE name LIKE $1 END
              AND CASE WHEN $2::BIGINT IS NULL THEN TRUE ELSE id > $2 END
            ORDER BY id ASC
            LIMIT $3
            "#,
            filter,
            after,
            first as i16
        )
        .fetch_all(&self.pool)
        .await
        .map_err(SteamStoreError::Sqlx)
    }
}
