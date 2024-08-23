use diesel::prelude::*; // Importing prelude also imports RunQueryDsl and other useful traits
use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

use crate::{
    agent::{Agent, NewAgent},
    db::schema::{agents, resource_specifications},
    graphql::context::Context,
    // resource_specification::{NewResourceSpecification, ResourceSpecification, ResourceType},
};

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    fn create_agent(context: &Context, name: String, note: Option<String>) -> FieldResult<Agent> {
        let conn = &mut context.pool.get().expect("Failed to get DB connection from pool");

        // Create the new agent instance
        let new_agent = NewAgent {
            name: &name,
            note: note.as_deref(),
        };

        // Insert the new agent into the database
        let inserted_agent = diesel::insert_into(agents::table)
            .values(&new_agent)
            .get_result(conn)?;

        Ok(inserted_agent)
    }

    // fn create_resource_specification(
    //     context: &Context,
    //     agent_id: Uuid, 
    //     name: String,
    //     note: Option<String>,
    //     resource_type: ResourceType
    // ) -> FieldResult<ResourceSpecification> {
    //     let conn = &mut context.pool.get().expect("Failed to get DB connection from pool");

    //     // Create the new resource specification instance
    //     let new_resource_spec = NewResourceSpecification {
    //         agent_id: &agent_id,
    //         name: &name,
    //         note: note.as_deref(),
    //         resource_type: &resource_type
    //     };

    //     // Insert the new resource specification into the database
    //     let inserted_resource_spec = diesel::insert_into(resource_specifications::table)
    //         .values(&new_resource_spec)
    //         .get_result(conn)?;

    //     Ok(inserted_resource_spec)
    // }
}
