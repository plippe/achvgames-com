pub mod api_steampowered_com;
pub mod environment;
pub mod store;
pub mod worker;

use crate::graphql::{Cursor, Page, PageInfo};
use crate::steam::store::games::SteamGamesStore;
use crate::utils::PipeExt;
use async_graphql::SimpleObject;

pub fn try_from_cursor(cursor: Cursor) -> Option<u32> {
    base64::decode_config(cursor.0, base64::URL_SAFE_NO_PAD)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .and_then(|str| str.parse::<u32>().ok())
}

pub fn into_cursor(id: u32) -> Cursor {
    base64::encode_config(id.to_string(), base64::URL_SAFE_NO_PAD).pipe(Cursor)
}

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, SimpleObject)]
pub struct Game {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct GameAchievement {
    pub name: String,
    pub description: Option<String>,
    pub icon_locked_url: String,
    pub icon_unlocked_url: String,
}

#[derive(Debug, Clone)]
pub struct GameWithAchievements {
    pub game: Game,
    pub achievements: Vec<GameAchievement>,
}

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn games(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(default = 10, validator(minimum = 10, maximum = 10))] first: usize,
        after: Option<Cursor>,
    ) -> Page<Game> {
        ctx.data_unchecked::<SteamGamesStore>()
            .get(first, after.and_then(try_from_cursor))
            .await
            .unwrap_or_default()
            .pipe(|nodes| Page {
                nodes: nodes.clone(),
                page_info: PageInfo {
                    has_next_page: nodes.len() == first,
                    end_cursor: nodes.last().map(|game| into_cursor(game.id)),
                },
            })
    }
}
