use async_graphql::{Context, Object};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::graphql::types::response::Response;
pub struct RootMutation;

#[Object]
impl RootMutation {
    pub async fn create_item<'a>(&self, ctx: &Context<'a>, name: String) -> Option<Response> {
        let db = ctx.data_unchecked::<Surreal<Client>>();

        let mut response = db
            .query(format!("CREATE test SET prop = $prop"))
            .bind(("prop", name))
            .await
            .unwrap();
        let result: Option<Response> = response.take(0).unwrap();
        result
    }
}
