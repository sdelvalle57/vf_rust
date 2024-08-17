use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::schema::agents;

#[derive(Queryable, Selectable, Debug)]
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

#[cfg(test)]
mod tests {
    use crate::db::conn::establish_connection;

    #[test]
    fn test_insert_agent() {
        use crate::agent::Agent;

        let conn = & mut establish_connection();

        let new_agent = Agent::new(conn, "Santiago".to_string(), Some("Santiagos".to_string()));

        match new_agent {
            Ok(agent) => {
                assert_eq!(agent.name, "Santiago");
                assert_eq!(agent.note, Some("Santiagos".to_string()));
                println!("Successfully created agent: {:?}", agent);
            }
            Err(e) => {
                eprintln!("Error creating agent: {}", e);
            }
        }

    }
}
