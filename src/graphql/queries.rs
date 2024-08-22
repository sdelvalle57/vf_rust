use crate::{
    agent::Agent,
    db::schema::{agents, resource_specifications},
    graphql::context::Context,
    resource_specification::ResourceSpecification,
};
use diesel::prelude::*;
use diesel::RunQueryDsl; // Specifically import RunQueryDsl to get access to the load method
use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    /*** Agents */
    fn all_agents(context: &Context) -> FieldResult<Vec<Agent>> {
        let conn = &mut context
            .pool
            .get()
            .expect("Failed to get DB connection from pool");

        let results = agents::table.load::<Agent>(conn)?;
        Ok(results)
    }

    fn agent_by_id(context: &Context, agent_id: Uuid) -> FieldResult<Agent> {
        let conn = &mut context
            .pool
            .get()
            .expect("Failed to get DB connection from pool");

        let results = agents::table
            .filter(agents::id.eq(agent_id))
            .first::<Agent>(conn)?;

        Ok(results)
    }

    /*** Resource Specifications */
    fn all_resource_specifications(context: &Context) -> FieldResult<Vec<ResourceSpecification>> {
        let conn = &mut context
            .pool
            .get()
            .expect("Failed to get DB connection from pool");

        let results = resource_specifications::table.load::<ResourceSpecification>(conn)?; // The `load` method is now available

        Ok(results)
    }

    fn resource_specifications_by_agent(
        context: &Context,
        agent_id: Uuid,
    ) -> FieldResult<Vec<ResourceSpecification>> {
        let conn = &mut context
            .pool
            .get()
            .expect("Failed to get DB connection from pool");

        let results = resource_specifications::table
            .filter(resource_specifications::agent_id.eq(agent_id))
            .load::<ResourceSpecification>(conn)?;

        Ok(results)
    }

    fn resource_specifications_by_id(
        context: &Context,
        resource_specification: Uuid,
    ) -> FieldResult<ResourceSpecification> {
        let conn = &mut context
            .pool
            .get()
            .expect("Failed to get DB connection from pool");

        let results = resource_specifications::table
            .filter(resource_specifications::id.eq(resource_specification))
            .first::<ResourceSpecification>(conn)?;

        Ok(results)
    }

    /*** Economic Resources */
}
