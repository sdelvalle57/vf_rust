use crate::agent::{Agent, NewAgent};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use juniper::{FieldResult, GraphQLObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {

    fn new_agent(name: String, note: Option<String>) -> FieldResult<Agent> {
        let agent = NewAgent::new(&name, note.as_deref());

        let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
        let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();

        let dt = NaiveDateTime::new(d, t);

        Ok(Agent {
            created_at: dt,
            id: Uuid::new_v4(),
            name: name.to_string(),
            note: note.map(|s| s.to_string()),
        })
    }
}
