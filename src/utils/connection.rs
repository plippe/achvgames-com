use crate::steam;
use async_graphql::OutputType;
use async_graphql::{scalar, SimpleObject};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chaining::Pipe;

#[derive(SimpleObject)]
#[graphql(concrete(name = "PageSteamGame", params(steam::Game)))]
pub struct Connection<A: OutputType> {
    pub edges: Vec<Edge<A>>,
    pub page_info: PageInfo,
}

#[derive(SimpleObject)]
pub struct Edge<A: OutputType> {
    pub node: A,
    pub cursor: Cursor,
}

#[derive(SimpleObject)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: Option<Cursor>,
    pub end_cursor: Option<Cursor>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Cursor(pub String);
scalar!(Cursor);

impl Cursor {
    pub fn try_into_i64(self) -> Option<i64> {
        URL_SAFE_NO_PAD
            .decode(self.0)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .and_then(|str| str.parse::<i64>().ok())
    }

    pub fn from_i64(value: i64) -> Cursor {
        URL_SAFE_NO_PAD.encode(value.to_string()).pipe(Cursor)
    }
}
