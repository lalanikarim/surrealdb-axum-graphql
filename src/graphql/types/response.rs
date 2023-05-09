use async_graphql::Object;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Response {
    pub id: Thing,
    pub prop: String,
}

#[Object]
impl Response {
    pub async fn id(&self) -> Option<&str> {
        if let Id::String(id) = &self.id.id {
            Some(id)
        } else {
            None
        }
    }
    pub async fn prop(&self) -> &str {
        &self.prop
    }
}
