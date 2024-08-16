use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use crate::graphql::query::Query;

pub type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> MySchema {
    Schema::build(Query, EmptyMutation, EmptySubscription).finish()
}