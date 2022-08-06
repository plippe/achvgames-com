use crate::steam;
use async_graphql::OutputType;
use async_graphql::{scalar, SimpleObject};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cursor(pub String);
scalar!(Cursor);

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
