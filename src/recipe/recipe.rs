use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::db::schema::{recipes, recipe_resources}; // Assuming you have these tables in your schema

#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = recipe_resources)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeResource {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub resource_specification_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = recipes)]
pub struct NewRecipe<'a> {
    pub agent_id: Uuid,
    pub name: &'a str,
    pub note: Option<&'a str>,
}

#[derive(Insertable)]
#[diesel(table_name = recipe_resources)]
pub struct NewRecipeResource {
    pub recipe_id: Uuid,
    pub resource_specification_id: Uuid,
}

impl<'a> NewRecipe<'a> {
    pub fn new(
        agent_id: Uuid,
        name: &'a str,
        note: Option<&'a str>,
    ) -> Self {
        NewRecipe {
            agent_id,
            name,
            note,
        }
    }
}

impl NewRecipeResource {
    pub fn new(
        recipe_id: Uuid,
        resource_specification_id: Uuid,
    ) -> Self {
        NewRecipeResource {
            recipe_id,
            resource_specification_id,
        }
    }
}
