
use diesel::{
    Insertable, 
    Queryable
};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::db::schema::recipe_templates;


use super::{recipe_flow_template::{ActionType, RecipeFlowTemplateWithDataFields}, recipe_template_blacklist::RecipeTemplateBlacklist};


#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = recipe_templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct RecipeTemplate {
    pub id: Uuid,
    pub map_template_id: Uuid,
    pub identifier: String,
    pub name: String,
    pub commitment: Option<ActionType>,
    pub fulfills: Option<Uuid>,
    pub trigger: Option<ActionType>
}


#[derive(Insertable)]
#[diesel(table_name = recipe_templates)]
pub struct NewRecipeTemplate<'a> {
    pub map_template_id: &'a Uuid,
    pub identifier: &'a str,
    pub name: &'a str,
    pub commitment: Option<&'a ActionType>,
    pub fulfills: Option<&'a Uuid>,
    pub trigger: Option<&'a ActionType>
}

impl<'a> NewRecipeTemplate<'a> {
    pub fn new(
        map_template_id: &'a Uuid,
        identifier: &'a str,
        name: &'a str,
        commitment: Option<&'a ActionType>,
        fulfills: Option<&'a Uuid>,
        trigger: Option<&'a ActionType>
    ) -> Self {
        NewRecipeTemplate {
            map_template_id,
            identifier,
            name,
            commitment,
            fulfills,
            trigger
        }
    }
}

#[derive(juniper::GraphQLObject, Debug)]
pub struct RecipeTemplateWithRecipeFlows {
    pub id: Uuid,
    pub map_template_id: Uuid,
    pub name: String,
    pub commitment: Option<ActionType>,
    pub fulfills: Option<Uuid>,
    pub identifier: String,
    pub trigger: Option<ActionType>,
    pub recipe_flows: Vec<RecipeFlowTemplateWithDataFields>
}

impl RecipeTemplateWithRecipeFlows {
    pub fn new(recipe_template: & RecipeTemplate) -> Self {
        RecipeTemplateWithRecipeFlows {
            map_template_id: recipe_template.map_template_id,
            id: recipe_template.id,
            name: recipe_template.name.clone(),
            commitment: recipe_template.commitment,
            fulfills: recipe_template.fulfills,
            identifier: recipe_template.identifier.clone(),
            trigger: recipe_template.trigger,
            recipe_flows: Vec::new()
        }
    }

    pub fn add_recipe_flow(&mut self, recipe_flow: RecipeFlowTemplateWithDataFields) {
        self.recipe_flows.push(recipe_flow)
    }
}