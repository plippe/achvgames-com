use async_graphql::{InputObject, SimpleObject};

#[derive(Debug, Clone, SimpleObject, InputObject)]
pub struct StringFilter {
    pub contains: Option<String>,
}
