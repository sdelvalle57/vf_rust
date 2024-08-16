use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::agents;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Debug, QueryableByName)]
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

impl Agent {
    pub fn new(
        conn: &mut PgConnection,  // Changed to mutable reference
        name: String,
        note: Option<String>,
    ) -> QueryResult<Agent> {

        let new_agent = NewAgent {
            name: &name,
            note: note.as_deref(),  // Convert Option<String> to Option<&str>
        };

        diesel::insert_into(agents::table)
            .values(&new_agent)
            .get_result::<Agent>(conn)  // Using mutable reference
    }
}
