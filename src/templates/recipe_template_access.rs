use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::db::schema::recipe_templates_access;

#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = recipe_templates_access)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeTemplateAccess {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub recipe_template_id: Uuid
}

#[derive(Insertable)]
#[diesel(table_name = recipe_templates_access)]
pub struct NewRecipeTemplateAccess<'a> {
    pub agent_id: &'a Uuid,
    pub recipe_template_id: &'a Uuid
}

impl<'a> NewRecipeTemplateAccess<'a> {
    pub fn new(
        agent_id: &'a Uuid,
        recipe_template_id: &'a Uuid
    ) -> Self {
        NewRecipeTemplateAccess {
            agent_id,
            recipe_template_id,
        }
    }
}

