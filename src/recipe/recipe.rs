// Process Map

use chrono::{DateTime, Utc};

use crate::agent::Agent;
use crate::resource_specification::{self, ResourceSpecification};

pub struct Recipe {
    pub id: String,
    pub agent: Agent,
    pub name: String,
    pub note: String,
    pub created_at: DateTime<Utc>,
}

pub struct RecipeResource {
    pub id: String,
    pub recipe: Recipe,
    pub resource_specification: ResourceSpecification
}

impl Recipe {
    pub fn new(
        id: &str,
        agent: Agent,
        name: &str,
        note: &str,
    ) -> Recipe {
        Recipe {
            id: id.to_string(),
            agent,
            name: name.to_string(),
            note: note.to_string(),
            created_at: Utc::now()
        }
    }
}

impl RecipeResource {
    pub fn new(id: &str, recipe: Recipe, resource_specification: ResourceSpecification) -> RecipeResource {
        RecipeResource {
            id: id.to_string(),
            recipe,
            resource_specification
        }
    }
}
