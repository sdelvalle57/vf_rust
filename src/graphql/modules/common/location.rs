
use crate::{
    common::location::{Location, NewLocation}, db::schema::locations, graphql::context::Context
};
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;


/**** Queries */
pub fn locations_by_agent(context: &Context, agent_id: Uuid) -> FieldResult<Vec<Location>> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let results = locations::table
        .filter(locations::agent_id.eq(agent_id))
        .load::<Location>(conn)?;

    Ok(results)
}


/*** Mutations */
pub fn create_location(context: &Context, agent_id: Uuid, name: String, value: String) -> FieldResult<Location> {
    let conn = &mut context.pool.get().expect("Failed to get DB connection from pool");

    // Create the new agent instance
    let new_location = NewLocation::new(&name, &agent_id, &value);

    // Insert the new agent into the database
    let inserted_agent = diesel::insert_into(locations::table)
        .values(&new_location)
        .get_result(conn)?;

    Ok(inserted_agent)
}