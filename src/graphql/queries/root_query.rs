use async_graphql::{Context, Object};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::graphql::types::response::Response;
use crate::graphql::types::search::Search;

pub struct RootQuery;

#[Object]
impl RootQuery {
    pub async fn items<'a>(&self, ctx: &Context<'a>) -> Vec<Response> {
        let db = ctx.data_unchecked::<Surreal<Client>>();

        let mut response = db.query("SELECT * FROM test").await.unwrap();
        let result: Vec<Response> = response.take(0).unwrap();
        result
    }
    pub async fn get_item<'a>(&self, ctx: &Context<'a>, id: String) -> Option<Response> {
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
    pub async fn search<'a>(&self, ctx: &Context<'a>, search: Search) -> Vec<Response> {
        let db = ctx.data_unchecked::<Surreal<Client>>();

        let mut response = db
            .query("SELECT * FROM test where prop = $prop")
            .bind(("prop", search.prop))
            .await
            .unwrap();
        response.take(0).unwrap()
    }
}
