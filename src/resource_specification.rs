use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::db::schema::resource_specifications;

#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = resource_specifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ResourceSpecification {
    pub id: Uuid,
    pub agent_id: Uuid,  // Field is no longer nullable
    pub name: String,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = resource_specifications)]
pub struct NewResourceSpecification<'a> {
    pub agent_id: &'a Uuid,
    pub name: &'a str,
    pub note: Option<&'a str>,
}

impl<'a> NewResourceSpecification<'a> {
    pub fn new(
        agent_id: &'a Uuid,
        name: &'a str,
        note: Option<&'a str>,
    ) -> Self {
        NewResourceSpecification {
            agent_id,
            name,
            note,
        }
    }
}
