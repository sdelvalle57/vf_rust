
use crate::{
    common::agent::{Agent, AgentWithLocations, NewAgent}, db::schema::agents, graphql::context::Context
};
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;

use super::location::locations_by_agent;


/**** Queries */
pub fn all_agents(context: &Context) -> FieldResult<Vec<Agent>> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let results = agents::table.load::<Agent>(conn)?;
    Ok(results)
}

pub fn agent_by_id(context: &Context, agent_id: Uuid) -> FieldResult<Agent> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let results = agents::table
        .filter(agents::id.eq(agent_id))
        .first::<Agent>(conn)?;

    Ok(results)
}


pub fn agents_with_location(context: &Context) -> FieldResult<Vec<AgentWithLocations>> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let agents = agents::table.load::<Agent>(conn)?;
    let mut results = Vec::new();
    for agent in agents {
        let locations = locations_by_agent(&context, agent.id)?;
        let agent_with_locations = AgentWithLocations::new(agent.id, agent.name, locations);
        results.push(agent_with_locations);
    }
    Ok(results)
}


/*** Mutations */

pub fn create_agent(context: &Context, name: String, note: Option<String>) -> FieldResult<Agent> {
    let conn = &mut context.pool.get().expect("Failed to get DB connection from pool");

    // Create the new agent instance
    let new_agent = NewAgent::new(&name, note.as_deref());

    // Insert the new agent into the database
    let inserted_agent = diesel::insert_into(agents::table)
        .values(&new_agent)
        .get_result(conn)?;

    Ok(inserted_agent)
}
