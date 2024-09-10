use diesel::prelude::{Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::{
    templates::recipe_template::RecipeTemplateType,
    db::schema::recipe_processes
};

use super::flow::RecipeProcessFlowResponse;

#[derive(Queryable, GraphQLObject, Debug, Clone)]
#[diesel(table_name = recipe_processes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeProcess {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub recipe_template_id: Option<Uuid>,
    pub name: String,
    pub recipe_type: RecipeTemplateType,
    pub output_of: Option<Uuid>
}

#[derive(Insertable)]
#[diesel(table_name = recipe_processes)]
pub struct NewRecipeProcess<'a> {
    pub recipe_id: &'a Uuid,
    pub recipe_template_id: &'a Uuid,
    pub name: &'a str,
    pub recipe_type: &'a RecipeTemplateType,
    pub output_of: Option<&'a Uuid>
}

impl<'a>  NewRecipeProcess<'a> {
    pub fn new(
        recipe_id: &'a Uuid,
        recipe_template_id: &'a Uuid,
        name: &'a str,
        recipe_type: &'a RecipeTemplateType,
        output_of: Option<&'a Uuid>
    ) -> Self {
        NewRecipeProcess {
            recipe_id,
            recipe_template_id, 
            name,
            recipe_type,
            output_of
        }
    }
}

#[derive(GraphQLObject)]
pub struct RecipeProcessResponse {
    pub id: Uuid,
    pub name: String,
    pub recipe_type: RecipeTemplateType,
    pub output_of: Option<Uuid>,
    pub process_flows: Vec<RecipeProcessFlowResponse>
}

impl RecipeProcessResponse {
    pub fn new(recipe_process: RecipeProcess) -> Self {
        RecipeProcessResponse {
            id: recipe_process.id,
            name: recipe_process.name,
            recipe_type: recipe_process.recipe_type,
            output_of: recipe_process.output_of,
            process_flows: Vec::new()
        }
    }
    pub fn add_recipe_process_flow(&mut self, recipe_process_flow: RecipeProcessFlowResponse) {
        self.process_flows.push(recipe_process_flow)
    }
}