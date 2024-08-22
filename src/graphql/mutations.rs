use diesel::prelude::*; // Importing prelude also imports RunQueryDsl and other useful traits
use juniper::{graphql_object, FieldResult};
use crate::{agent::{Agent, NewAgent}, db::schema::agents, graphql::context::Context};

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    fn new_agent(context: &Context, name: String, note: Option<String>) -> FieldResult<Agent> {
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
}
