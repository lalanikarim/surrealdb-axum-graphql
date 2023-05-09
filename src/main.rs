mod graphql;

use std::error::Error;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Extension, Router, Server,
};

use graphql::queries::root_query::RootQuery;
use graphql::schema::MySchema;

use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

#[macro_use]
extern crate dotenv_codegen;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

async fn graphql_handler(schema: Extension<MySchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = Surreal::new::<Ws>(dotenv!("SURREALDB_URL")).await?;

    db.signin(Root {
        username: dotenv!("SURREALDB_USERNAME"),
        password: dotenv!("SURREALDB_PASSWORD"),
    })
    .await?;

    db.use_ns(dotenv!("SURREALDB_NS"))
        .use_db(dotenv!("SURREALDB_DATABASE"))
        .await?;

    let schema = Schema::build(RootQuery, EmptyMutation, EmptySubscription)
        .data(db)
        .finish();

    let app: Router = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    let addr = dotenv!("API_LISTEN_ON");

    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
