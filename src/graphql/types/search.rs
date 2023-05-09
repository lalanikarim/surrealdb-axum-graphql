use async_graphql::InputObject;
use serde::Deserialize;

#[derive(Debug, Deserialize, InputObject)]
pub struct Search {
    pub prop: String,
}
