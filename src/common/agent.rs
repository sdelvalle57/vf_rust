use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::db::schema::agents;
use super::location::Location;

#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = agents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = agents)]
pub struct NewAgent<'a> {
    pub name: &'a str,
    pub note: Option<&'a str>,
}

impl<'a> NewAgent<'a> {
    pub fn new(
        name: &'a str,
        note: Option<&'a str>,
    ) -> Self {
        NewAgent {
            name,
            note,
        }
    }
}

#[derive(juniper::GraphQLObject, Debug)]
pub struct AgentLocation {
    pub id: Uuid,
    pub name: String,
    pub value: String
}

#[derive(juniper::GraphQLObject, Debug)]
pub struct AgentWithLocations {
    pub id: Uuid,
    pub name: String,
    pub locations: Vec<AgentLocation>
}

impl AgentWithLocations {
    pub fn new(id: Uuid, name: String, locations: Vec<Location>) -> Self {
        AgentWithLocations {
            id,
            name,
            locations: locations.into_iter().map(|l: Location| AgentLocation {
                id: l.id,
                name: l.name,
                value: l.value
            }).collect(), // Collect the iterator back into a Vec
        }
    }
}
