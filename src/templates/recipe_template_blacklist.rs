
use diesel::{
    Insertable, 
    Queryable
};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::db::schema::recipe_template_blacklists;


#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = recipe_template_blacklists)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct RecipeTemplateBlacklist {
    pub id: Uuid,
    pub map_template_id: Uuid,
    pub recipe_template_id: Uuid,
    pub recipe_template_predecesor_id: Uuid
}

#[derive(Insertable)]
#[diesel(table_name = recipe_template_blacklists)]
pub struct NewRecipeTemplateBlacklist<'a> {
    pub map_template_id: &'a Uuid,
    pub recipe_template_id: &'a Uuid,
    pub recipe_template_predecesor_id: &'a Uuid
}

impl<'a> NewRecipeTemplateBlacklist<'a> {
    pub fn new(
        map_template_id: &'a Uuid,
        recipe_template_id: &'a Uuid,
        recipe_template_predecesor_id: &'a Uuid
    ) -> Self {
        NewRecipeTemplateBlacklist {
            map_template_id,
            recipe_template_id,
            recipe_template_predecesor_id
        }
    }
}