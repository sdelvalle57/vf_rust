use juniper::{EmptySubscription, RootNode};
use crate::graphql::queries::QueryRoot;
use crate::graphql::mutations::MutationRoot;
use crate::graphql::context::Context;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}