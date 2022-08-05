use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Object, Schema,
};
use tide::{http::mime, Body, Response, StatusCode};

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn foo(&self) -> String {
        "bar".to_owned()
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let mut app = tide::new();

    app.at("/").get(|_| async move {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_body(Body::from_string(playground_source(
            GraphQLPlaygroundConfig::new("/graphql"),
        )));
        resp.set_content_type(mime::HTML);
        Ok(resp)
    });

    app.at("/graphql").post(async_graphql_tide::graphql(schema));

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
