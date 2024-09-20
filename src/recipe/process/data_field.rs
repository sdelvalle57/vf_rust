use diesel::prelude::{Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::{db::schema::recipe_process_flow_data_fields, templates::recipe_flow_template_data_field::{FieldClass, FieldType, FlowThrough}};

#[derive(Queryable, GraphQLObject, Debug, Clone)]
#[diesel(table_name = recipe_process_flow_data_fields)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeFlowDataField {
    pub id: Uuid,
    pub recipe_process_flow_id: Uuid,
    pub recipe_flow_template_data_field_id: Option<Uuid>,
    pub field_identifier: String,
    pub field_class: FieldClass,
    pub field: String,
    pub field_type: FieldType,
    pub note: Option<String>,
    pub required: bool,
    pub default_value: Option<String>,
    pub flow_through: Option<FlowThrough>,
}


#[derive(Insertable)]
#[diesel(table_name = recipe_process_flow_data_fields)]
pub struct NewRecipeFlowDataField<'a> {
    pub recipe_process_flow_id: &'a Uuid,
    pub recipe_flow_template_data_field_id: Option<&'a Uuid>,
    pub field_identifier: &'a str,
    pub field_class: &'a FieldClass,
    pub field: &'a str,
    pub field_type: &'a FieldType,
    pub note: Option<&'a str>,
    pub required: bool,
    pub default_value: Option<&'a str>,
    pub flow_through: Option<&'a FlowThrough>,
}

impl<'a>  NewRecipeFlowDataField<'a> {
    pub fn new(
        recipe_process_flow_id: &'a Uuid,
        recipe_flow_template_data_field_id: Option<&'a Uuid>,
        field_identifier: &'a str,
        field_class: &'a FieldClass,
        field: &'a str,
        field_type: &'a FieldType,
        note: Option<&'a str>,
        required: bool,
        default_value: Option<&'a str>,
        flow_through: Option<&'a FlowThrough>,
    ) -> Self {
        NewRecipeFlowDataField {
            recipe_process_flow_id,
            recipe_flow_template_data_field_id, 
            field_identifier,
            field_class,
            field,
            field_type,
            note,
            required,
            default_value,
            flow_through
        }
    }
}