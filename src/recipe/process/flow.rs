use diesel::prelude::{Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::{db::schema::recipe_process_flows, templates::recipe_flow_template::{ActionType, EventType, RoleType}};

use super::data_field::RecipeFlowDataField;

#[derive(Queryable, GraphQLObject, Debug, Clone)]
#[diesel(table_name = recipe_process_flows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeProcessFlow {
    pub id: Uuid,
    pub recipe_process_id: Uuid,
    pub recipe_flow_template_id: Uuid,
    pub event_type: EventType,
    pub role_type: RoleType,
    pub action: ActionType
}


#[derive(Insertable)]
#[diesel(table_name = recipe_process_flows)]
pub struct NewRecipeProcessFlow<'a> {
    pub recipe_process_id: &'a Uuid,
    pub recipe_flow_template_id: &'a Uuid,
    pub event_type: &'a EventType,
    pub role_type: &'a RoleType,
    pub action: &'a ActionType
}

impl<'a>  NewRecipeProcessFlow<'a> {
    pub fn new(
        recipe_process_id: &'a Uuid,
        recipe_flow_template_id: &'a Uuid,
        event_type: &'a EventType,
        role_type: &'a RoleType,
        action: &'a ActionType
    ) -> Self {
        NewRecipeProcessFlow {
            recipe_process_id,
            recipe_flow_template_id, 
            event_type,
            role_type,
            action
        }
    }
}


#[derive(GraphQLObject)]
pub struct RecipeProcessFlowResponse {
    pub id: Uuid,
    pub event_type: EventType,
    pub role_type: RoleType,
    pub action: ActionType,
    pub data_fields: Vec<RecipeFlowDataField>
}

impl RecipeProcessFlowResponse {
    pub fn new(recipe_process_flow: RecipeProcessFlow) -> Self {
        RecipeProcessFlowResponse {
            id: recipe_process_flow.id,
            event_type: recipe_process_flow.event_type,
            role_type: recipe_process_flow.role_type,
            action: recipe_process_flow.action,
            data_fields: Vec::new()
        }
    }

    pub fn add_data_field(&mut self, data_field: RecipeFlowDataField) {
        self.data_fields.push(data_field)
    }
}