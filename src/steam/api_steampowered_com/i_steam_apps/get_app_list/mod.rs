use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(rename = "applist")]
    pub app_list: ResponseAppList,
}

#[derive(Deserialize, Debug)]
pub struct ResponseAppList {
    pub apps: Vec<ResponseApp>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseApp {
    #[serde(rename = "appid")]
    pub app_id: i64,
}

pub struct Request {
    pub key: String,
}

impl crate::steam::api_steampowered_com::Request for Request {
    fn uri(&self) -> String {
        format!(
            "https://api.steampowered.com/ISteamApps/GetAppList/v2/?format=json&key={}",
            self.key
        )
    }
}
