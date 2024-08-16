use chrono::{DateTime, Utc};

use crate::agent::Agent;

pub struct ResourceSpecification {
    pub id: String,
    pub agent: Agent,
    pub name: String,
    pub note: String,
    pub created_at: DateTime<Utc>
}

impl ResourceSpecification {
    pub fn new(id: &str, agent: Agent, name: &str, note: &str) -> ResourceSpecification {
        ResourceSpecification{
            id: id.to_string(),
            agent,
            name: name.to_string(),
            note: note.to_string(),
            created_at: Utc::now()
        }
    }
}