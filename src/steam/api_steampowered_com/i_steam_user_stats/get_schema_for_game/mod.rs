use crate::steam::{Game, GameAchievement, GameWithAchievements};
use itertools::Itertools;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub game: ResponseGame,
}

#[derive(Deserialize, Debug)]
pub struct ResponseGame {
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "availableGameStats")]
    pub available_game_stats: ResponseGameStats,
}

#[derive(Deserialize, Debug)]
pub struct ResponseGameStats {
    pub achievements: Vec<ResponseAchievement>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseAchievement {
    #[serde(rename = "displayName")]
    pub name: String,
    pub description: Option<String>,
    pub icon: String,
    #[serde(rename = "icongray")]
    pub icon_gray: String,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub key: String,
    pub app_id: i64,
}

impl crate::steam::api_steampowered_com::Request for Request {
    fn uri(&self) -> String {
        format!("http://api.steampowered.com/ISteamUserStats/GetSchemaForGame/v2/?format=json&key={}&appid={}", self.key, self.app_id)
    }
}

impl GameWithAchievements {
    pub fn from_request_and_response(req: &Request, res: &Response) -> GameWithAchievements {
        GameWithAchievements {
            game: Game {
                id: req.app_id,
                name: res.game.game_name.clone(),
            },
            achievements: res
                .game
                .available_game_stats
                .achievements
                .iter()
                .map(|achv| GameAchievement {
                    name: achv.name.clone(),
                    description: achv.description.clone(),
                    icon_locked_url: achv.icon_gray.clone(),
                    icon_unlocked_url: achv.icon.clone(),
                })
                .collect_vec(),
        }
    }
}
