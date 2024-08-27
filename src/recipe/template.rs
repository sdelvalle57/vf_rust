use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::db::schema::templates;

#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Template {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = templates)]
pub struct NewTemplate<'a> {
    pub name: &'a str
}

impl<'a> NewTemplate<'a> {
    pub fn new(
        name: &'a str
    ) -> Self {
        NewTemplate {
            name
        }
    }
}