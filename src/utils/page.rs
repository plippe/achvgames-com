use crate::steam;
use chaining::Pipe;
use async_graphql::OutputType;
use async_graphql::{scalar, SimpleObject};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

#[derive(SimpleObject)]
#[graphql(concrete(name = "PageSteamGame", params(steam::Game)))]
pub struct Page<A: OutputType> {
    pub nodes: Vec<A>,
    pub page_info: PageInfo,
}

#[derive(SimpleObject)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<Cursor>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cursor(pub String);
scalar!(Cursor);

impl Cursor {
    pub fn try_into_u32(self) -> Option<u32> {
        URL_SAFE_NO_PAD.decode(self.0)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .and_then(|str| str.parse::<u32>().ok())
    }

    pub fn from_u32(value: u32) -> Cursor {
        URL_SAFE_NO_PAD.encode(value.to_string()).pipe(Cursor)
    }
}
