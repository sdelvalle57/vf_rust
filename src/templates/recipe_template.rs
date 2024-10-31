
use diesel::{
    Insertable, 
    Queryable
};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::db::schema::recipe_templates;

use super::recipe_flow_template::{ActionType, RecipeFlowTemplateWithDataFields};


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
    pub trigger: Option<ActionType>,
    pub version: i32,
    pub overriden_by: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub first_version: Option<Uuid>
}


#[derive(Insertable)]
#[diesel(table_name = recipe_templates)]
pub struct NewRecipeTemplate<'a> {
    pub map_template_id: &'a Uuid,
    pub identifier: &'a str,
    pub name: &'a str,
    pub commitment: Option<&'a ActionType>,
    pub fulfills: Option<&'a Uuid>,
    pub trigger: Option<&'a ActionType>,
    pub version:i32,
    pub overriden_by: Option<&'a Uuid>,
    pub created_by: Option<&'a Uuid>,
    pub first_version: Option<&'a Uuid>
}

impl<'a> NewRecipeTemplate<'a> {
    pub fn new(
        map_template_id: &'a Uuid,
        identifier: &'a str,
        name: &'a str,
        commitment: Option<&'a ActionType>,
        fulfills: Option<&'a Uuid>,
        trigger: Option<&'a ActionType>,
        version: i32,
        overriden_by: Option<&'a Uuid>,
        created_by: Option<&'a Uuid>,
        first_version: Option<&'a Uuid>
    ) -> Self {
        NewRecipeTemplate {
            map_template_id,
            identifier,
            name,
            commitment,
            fulfills,
            trigger,
            version,
            overriden_by,
            created_by,
            first_version
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
    pub version: i32,
    pub overriden_by: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub first_version: Option<Uuid>,
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
            version: recipe_template.version,
            overriden_by: recipe_template.overriden_by,
            created_by: recipe_template.created_by,
            first_version: recipe_template.first_version,
            recipe_flows: Vec::new()
        }
    }

    pub fn add_recipe_flow(&mut self, recipe_flow: RecipeFlowTemplateWithDataFields) {
        self.recipe_flows.push(recipe_flow)
    }
}