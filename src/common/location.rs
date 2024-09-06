use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::db::schema::locations;

#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = agents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Location {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub name: String,
    pub value: String,
}

#[derive(Insertable)]
#[diesel(table_name = locations)]
pub struct NewLocation<'a> {
    pub agent_id: &'a Uuid,
    pub name: &'a str,
    pub value: &'a str,
}

impl<'a> NewLocation<'a> {
    pub fn new(
        name: &'a str,
        agent_id: &'a Uuid,
        value: &'a str
    ) -> Self {
        NewLocation {
            name,
            agent_id,
            value,
        }
    }
}

