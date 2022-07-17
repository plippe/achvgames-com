pub mod i_steam_apps;
pub mod i_steam_user_stats;

use serde::Deserialize;
use surf::Client as SurfClient;
use surf::StatusCode;

pub trait Request {
    fn uri(&self) -> String;
}

#[derive(Debug)]
pub enum SteamClientError {
    BadRequest,
    Forbidden,
    TooManyRequests,
    Unknown(u16, String),
}

pub struct SteamClient {
    pub http_client: SurfClient,
}
impl SteamClient {
    pub async fn request<Req, Res>(&self, req: &Req) -> Result<Res, SteamClientError>
    where
        Req: Request,
        Res: for<'de> Deserialize<'de>,
    {
        let mut res = self
            .http_client
            .get(req.uri())
            .send()
            .await
            .expect("Valid request");

        match (res.status(), res.body_json::<Res>().await) {
            (StatusCode::Ok, Ok(res)) => Ok(res),
            (StatusCode::Ok, Err(err)) => {
                println!("Failed to parse response: {}, {}", req.uri(), err);
                Err(SteamClientError::BadRequest)
            }
            (StatusCode::BadRequest, _) => Err(SteamClientError::BadRequest),
            (StatusCode::Forbidden, _) => Err(SteamClientError::Forbidden),
            (StatusCode::TooManyRequests, _) => Err(SteamClientError::TooManyRequests),
            (status, _) => Err(SteamClientError::Unknown(
                status.into(),
                res.body_string()
                    .await
                    .unwrap_or_else(|err| err.to_string()),
            )),
        }
    }
}
