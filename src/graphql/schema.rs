use async_graphql::{EmptySubscription, Schema};

use super::{mutations::root_mutation::RootMutation, queries::root_query::RootQuery};

pub type MySchema = Schema<RootQuery, RootMutation, EmptySubscription>;
