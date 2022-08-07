use crate::steam::api_steampowered_com::i_steam_apps::get_app_list;
use crate::steam::api_steampowered_com::i_steam_user_stats::get_schema_for_game;
use crate::steam::api_steampowered_com::{SteamClient, SteamClientError};
use crate::steam::environment;
use crate::steam::store::game_achievements::SteamGameAchievementsStore;
use crate::steam::store::games::SteamGamesStore;
use crate::steam::store::SteamStoreError;
use crate::steam::GameWithAchievements;
use crate::utils::pipe::PipeExt;
use futures::future;
use futures::stream::{self, StreamExt, TryStreamExt};
use itertools::Itertools;
use std::ops::Not;

#[derive(Debug)]
pub enum SteamWorkerError {
    Client(SteamClientError),
    Store(SteamStoreError),
}

impl std::error::Error for SteamWorkerError {}
impl std::fmt::Display for SteamWorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<SteamClientError> for SteamWorkerError {
    fn from(error: SteamClientError) -> Self {
        Self::Client(error)
    }
}

impl From<SteamStoreError> for SteamWorkerError {
    fn from(error: SteamStoreError) -> Self {
        Self::Store(error)
    }
}

pub struct SteamWorker {
    pub api_client: SteamClient,
    pub games_store: SteamGamesStore,
    pub game_achievements_store: SteamGameAchievementsStore,
}

impl SteamWorker {
    const MAX_CONCURRENCY: usize = 100;

    pub async fn work(&self) -> Result<(), SteamWorkerError> {
        println!("Getting memento");
        let memento = self.get_memento().await?.unwrap_or_default();
        println!("Getting memento - {}", memento);

        println!("Getting app ids");
        let app_ids = self
            .get_game_ids()
            .await?
            .cycle()
            .skip_while(|app_id| (app_id <= &memento).pipe(future::ready))
            .take_while(|app_id| (app_id != &memento).pipe(future::ready))
            .chain(memento.pipe(future::ready).pipe(stream::once));

        let count = app_ids
            .inspect(|app_id| println!("Getting game - {}", app_id))
            .map(|app_id| self.get_game(app_id))
            .buffer_unordered(Self::MAX_CONCURRENCY)
            .inspect_ok(|game_with_achv| println!("Setting app - {}", game_with_achv.game.id))
            .and_then(|game_with_achv| self.set_game(game_with_achv))
            .take_while(|res| self.is_critical(res).not().pipe(future::ready))
            .filter(|res| res.is_ok().pipe(future::ready))
            .count()
            .await;

        println!("Finished - {}", count);

        Ok(())
    }

    async fn get_memento(&self) -> Result<Option<u32>, SteamWorkerError> {
        self.games_store
            .get_last_updated()
            .await?
            .map(|game| game.id)
            .pipe(Ok)
    }

    async fn get_game_ids(
        &self,
    ) -> Result<futures::stream::Iter<std::vec::IntoIter<u32>>, SteamWorkerError> {
        let req = get_app_list::Request {
            key: environment::STEAM_WEB_API_KEY.to_owned(),
        };

        self.api_client
            .request::<get_app_list::Request, get_app_list::Response>(&req)
            .await?
            .app_list
            .apps
            .iter()
            .map(|app| app.app_id)
            .sorted()
            .pipe(stream::iter)
            .pipe(Ok)
    }

    async fn get_game(&self, app_id: u32) -> Result<GameWithAchievements, SteamWorkerError> {
        let req = get_schema_for_game::Request {
            key: environment::STEAM_WEB_API_KEY.to_owned(),
            app_id,
        };

        self.api_client
            .request::<get_schema_for_game::Request, get_schema_for_game::Response>(&req)
            .await?
            .pipe(|res| GameWithAchievements::from((req, res)))
            .pipe(Ok)
    }

    async fn set_game(
        &self,
        game_with_achievements: GameWithAchievements,
    ) -> Result<(), SteamWorkerError> {
        self.games_store
            .upsert(&game_with_achievements.game)
            .await?;

        self.game_achievements_store
            .upsert_bulk(
                game_with_achievements.game.id,
                &game_with_achievements.achievements,
            )
            .await?;

        Ok(())
    }

    fn is_critical(&self, res: &Result<(), SteamWorkerError>) -> bool {
        match res {
            Err(SteamWorkerError::Store(_)) => true,
            Err(SteamWorkerError::Client(SteamClientError::TooManyRequests)) => true,
            Err(SteamWorkerError::Client(SteamClientError::Unknown(status, body))) => {
                println!("Unknown steam client response: {}, {}", status, body);
                true
            }
            _ => false,
        }
    }
}
