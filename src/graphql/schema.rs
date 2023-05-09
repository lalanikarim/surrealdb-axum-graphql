use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use super::queries::root_query::RootQuery;

pub type MySchema = Schema<RootQuery, EmptyMutation, EmptySubscription>;
