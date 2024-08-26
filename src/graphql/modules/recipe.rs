
use crate::{
    agent::{Agent, NewAgent},
    db::schema::agents,
    graphql::context::Context, recipe::recipe::Recipe
};
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;

//TODO: first create recipe resources, then send a vec of ids of those here
pub fn recipe_by_id(context: &Context, recipe_id: Uuid) -> FieldResult<Recipe> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let results = agents::table
        .filter(agents::id.eq(agent_id))
        .first::<Agent>(conn)?;

    Ok(results)
}