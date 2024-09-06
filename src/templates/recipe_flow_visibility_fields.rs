use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::db::schema::recipe_flow_template_visibility_fields;

#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = recipe_flow_template_visibility_fields)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeFlowVisibilityField {
    pub id: Uuid,
    pub recipe_flow_template_id: Uuid,
    pub field_id: Uuid
}

#[derive(Insertable)]
#[diesel(table_name = recipe_flow_template_visibility_fields)]
pub struct NewRecipeFlowVisibilityField<'a> {
    pub recipe_flow_template_id: &'a Uuid,
    pub field_id: &'a Uuid
}

impl<'a> NewRecipeFlowVisibilityField<'a> {
    pub fn new(
        recipe_flow_template_id: &'a Uuid,
        field_id: &'a Uuid
    ) -> Self {
        NewRecipeFlowVisibilityField {
            recipe_flow_template_id,
            field_id,
        }
    }
}

