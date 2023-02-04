use crate::steam;
use chaining::Pipe;
use async_graphql::OutputType;
use async_graphql::{scalar, SimpleObject};

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
        base64::decode_config(self.0, base64::URL_SAFE_NO_PAD)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .and_then(|str| str.parse::<u32>().ok())
    }

    pub fn from_u32(value: u32) -> Cursor {
        base64::encode_config(value.to_string(), base64::URL_SAFE_NO_PAD).pipe(Cursor)
    }
}
