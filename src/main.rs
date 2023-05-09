use std::error::Error;

use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, InputObject, Object, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Extension, Router, Server,
};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::{Id, Thing},
    Surreal,
};

#[macro_use]
extern crate dotenv_codegen;

type MySchema = Schema<crate::Query, EmptyMutation, EmptySubscription>;

#[derive(Debug, Deserialize, InputObject)]
struct Search {
    prop: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
struct Response {
    id: Thing,
    prop: String,
}

#[Object]
impl Response {
    async fn id(&self) -> Option<&str> {
        if let Id::String(id) = &self.id.id {
            Some(id)
        } else {
            None
        }
    }
    async fn prop(&self) -> &str {
        &self.prop
    }
}

struct Query;

#[Object]
impl Query {
    async fn items<'a>(&self, ctx: &Context<'a>) -> Vec<Response> {
        let db = ctx.data_unchecked::<Surreal<Client>>();

        let mut response = db.query("SELECT * FROM test").await.unwrap();
        let result: Vec<Response> = response.take(0).unwrap();
        result
    }
    async fn create_item<'a>(&self, ctx: &Context<'a>, name: String) -> Option<Response> {
        let db = ctx.data_unchecked::<Surreal<Client>>();

        let mut response = db
            .query(format!("CREATE test SET prop = $prop"))
            .bind(("prop", name))
            .await
            .unwrap();
        let result: Option<Response> = response.take(0).unwrap();
        result
    }
    async fn get_item<'a>(&self, ctx: &Context<'a>, id: String) -> Option<Response> {
        let db = ctx.data_unchecked::<Surreal<Client>>();

        let mut response = db.query(format!("SELECT * FROM test:{id}")).await.unwrap();
        let result: Result<Option<Response>, surrealdb::Error> = response.take(0);

        match result {
            Ok(response) => response,
            Err(err) => {
                println!("Error: {err:?}");
                None
            }
        }
    }
    async fn search<'a>(&self, ctx: &Context<'a>, search: Search) -> Vec<Response> {
        let db = ctx.data_unchecked::<Surreal<Client>>();

        let mut response = db
            .query("SELECT * FROM test where prop = $prop")
            .bind(("prop", search.prop))
            .await
            .unwrap();
        response.take(0).unwrap()
    }
}

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

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
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
