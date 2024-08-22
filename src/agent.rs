use chrono::NaiveDateTime;
use diesel::{prelude::*, Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::db::schema::agents;

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

