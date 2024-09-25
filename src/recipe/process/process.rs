use diesel::prelude::{Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::{
    db::schema::{recipe_process_relations, recipe_processes}, templates::{recipe_flow_template::{ActionType, EventType, RoleType}, recipe_template::RecipeTemplateType}
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
    pub commitment: Option<ActionType>,
    pub fulfills: Option<Uuid>,
    pub recipe_type: RecipeTemplateType,
    pub identifier: String,
}


#[derive(Insertable)]
#[diesel(table_name = recipe_processes)]
pub struct NewRecipeProcess<'a> {
    pub recipe_id: &'a Uuid,
    pub recipe_template_id: &'a Uuid,
    pub name: &'a str,
    pub commitment: Option<&'a ActionType>,
    pub fulfills: Option<&'a Uuid>,
    pub recipe_type: &'a RecipeTemplateType,
    pub identifier: &'a str,
}

impl<'a>  NewRecipeProcess<'a> {
    pub fn new(
        recipe_id: &'a Uuid,
        recipe_template_id: &'a Uuid,
        name: &'a str,
        commitment: Option<&'a ActionType>,
        fulfills: Option<&'a Uuid>,
        recipe_type: &'a RecipeTemplateType,
        identifier: &'a str
    ) -> Self {
        NewRecipeProcess {
            recipe_id,
            recipe_template_id, 
            name,
            commitment,
            fulfills,
            recipe_type,
            identifier
        }
    }
}


#[derive(Queryable, GraphQLObject, Debug, Clone)]
#[diesel(table_name = recipe_process_relations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OutputOf {
    pub id: Uuid,
    pub recipe_process_id: Uuid,
    pub output_of: Uuid
}

#[derive(Insertable)]
#[diesel(table_name = recipe_process_relations)]
pub struct NewOutpuOf<'a> {
    pub recipe_process_id: &'a Uuid,
    pub output_of: &'a Uuid
}

impl<'a>  NewOutpuOf<'a> {
    pub fn new(
        recipe_process_id: &'a Uuid,
        output_of: &'a Uuid,
    ) -> Self {
        NewOutpuOf {
            recipe_process_id,
            output_of
        }
    }
}



#[derive(GraphQLObject)]
pub struct RecipeProcessResponse {
    pub id: Uuid,
    pub name: String,
    pub recipe_type: RecipeTemplateType,
    pub output_of: Vec<Uuid>,
    pub process_flows: Vec<RecipeProcessFlowResponse>,
    pub commitment: Option<ActionType>,
    pub fulfills: Option<Uuid>,
    pub identifier: String,
}

impl RecipeProcessResponse {
    pub fn new(recipe_process: RecipeProcess) -> Self {
        RecipeProcessResponse {
            id: recipe_process.id,
            name: recipe_process.name,
            recipe_type: recipe_process.recipe_type,
            commitment: recipe_process.commitment,
            fulfills: recipe_process.fulfills,
            identifier: recipe_process.identifier,
            output_of: Vec::new(),
            process_flows: Vec::new()
        }
    }

    pub fn add_recipe_process_flow(&mut self, recipe_process_flow: RecipeProcessFlowResponse) {
        self.process_flows.push(recipe_process_flow)
    }

    pub fn add_output_of(&mut self, id: Uuid) {
        self.output_of.push(id)
    }
}


#[derive(Queryable, GraphQLObject, Debug, Clone)]
#[diesel(table_name = recipe_process_flows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProcessFlow {
    pub id: Uuid,
    pub recipe_process_id: Uuid,
    pub recipe_flow_template_id: Uuid,
    pub event_type: EventType,
    pub role_type: RoleType,
    pub action: ActionType
}
