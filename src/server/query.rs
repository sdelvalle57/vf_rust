use async_graphql::{Context, Object, Result};
use diesel::prelude::*;
use uuid::Uuid;
use crate::db::establish_connection;
use crate::agent::Agent;

pub struct Query;

#[Object]
impl Query {
    async fn agent(&self, ctx: &Context<'_>, id: Uuid) -> Result<Agent> {
        let conn = &mut establish_connection();
        use crate::schema::agents::dsl::*;
        let result = agents.filter(id.eq(id))
            .first::<Agent>(conn)
            .expect("Error loading agent");
        Ok(result)
    }
}
