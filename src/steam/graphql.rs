use crate::steam::store::game_achievements::SteamGameAchievementsStore;
use crate::steam::store::games::GameFilter;
use crate::steam::store::games::SteamGamesStore;
use crate::steam::{Game, GameAchievement, GameImages};
use crate::utils::connection::{Connection, Cursor, Edge, PageInfo};
use chaining::Pipe;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn game(&self, ctx: &async_graphql::Context<'_>, id: u32) -> Option<Game> {
        ctx.data_unchecked::<SteamGamesStore>()
            .get_by_id(id)
            .await
            .ok()
            .flatten()
    }

    async fn games(
        &self,
        ctx: &async_graphql::Context<'_>,
        filter: Option<GameFilter>,
        #[graphql(default = 10, validator(minimum = 10, maximum = 10))] first: usize,
        after: Option<Cursor>,
    ) -> Connection<Game> {
        ctx.data_unchecked::<SteamGamesStore>()
            .get_all(filter, first, after.and_then(Cursor::try_into_u32))
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|game| Edge {
                cursor: Cursor::from_u32(game.id),
                node: game,
            })
            .collect::<Vec<Edge<Game>>>()
            .pipe(|edges| Connection {
                page_info: PageInfo {
                    has_next_page: edges.len() == first,
                    has_previous_page: false,
                    start_cursor: edges.first().map(|edge| edge.cursor.clone()),
                    end_cursor: edges.last().map(|edge| edge.cursor.clone()),
                },
                edges,
            })
    }
}

#[async_graphql::ComplexObject]
impl Game {
    async fn achievements(&self, ctx: &async_graphql::Context<'_>) -> Vec<GameAchievement> {
        ctx.data_unchecked::<SteamGameAchievementsStore>()
            .get_by_game_id(self.id)
            .await
            .unwrap_or_default()
    }

    async fn images(&self) -> GameImages {
        GameImages {
            header_url: format!(
                "https://cdn.cloudflare.steamstatic.com/steam/apps/{}/header.jpg",
                self.id
            ),
        }
    }
}
