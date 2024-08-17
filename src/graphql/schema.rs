use juniper::{EmptySubscription, RootNode};
use super::queries::QueryRoot;
use super::mutations::MutationRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
