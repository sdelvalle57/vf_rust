use crate::{
    db::schema::{recipe_flow_template_data_fields, recipe_flow_templates, recipe_templates}, 
    graphql::context::Context, 
    recipe::process::ProcessWithRecipe, 
    templates::{
        recipe_flow_template::{ActionType, EventType, NewRecipeFlowTemplate, RecipeFlowTemplate, RoleType}, 
        recipe_flow_template_data_field::{FieldType, FieldValue, NewRecipeFlowTemplateDataField}, 
        recipe_template::{NewRecipeTemplate, RecipeTemplate, RecipeTemplateType}
}, 
};
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;


struct RecipeFlowTemplateArg {
    event_type: EventType,
    role_type: RoleType,
    action: ActionType,
    data_fields: Vec<RecipeFlowTemplateDataFieldArg>
}

struct RecipeFlowTemplateDataFieldArg {
    pub field_value: FieldValue,
    pub field: String,
    pub field_type: FieldType,
    pub note: Option<String>,
    pub required: bool,
    pub query: Option<String>,
    pub default_value: Option<String>
}

//TODO: return the whole recipe flow
struct NewRecipeFlowResponse {
    // recipe_flow: 
}



/** Mutations */
pub fn create_recipe_template(
    context: &Context,
    name: String,
    recipe_template_type: RecipeTemplateType,
    recipe_flow_template_args: Vec<RecipeFlowTemplateArg>
) -> FieldResult<ProcessWithRecipe> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let new_template = NewRecipeTemplate::new(&name, &recipe_template_type);

    let inserted_template: RecipeTemplate = diesel::insert_into(recipe_templates::table)
        .values(&new_template)
        .get_result(conn)?;

    for r in recipe_flow_template_args {

        let new_recipe_flow_template  = NewRecipeFlowTemplate::new(&inserted_template.id, &r.event_type, &r.role_type, &r.action)
        let inserted_recipe_flow_template: RecipeFlowTemplate = diesel::insert_into(recipe_flow_templates::table)
            .values(&new_recipe_flow_template)
            .get_result(conn)?;

        for rd in r.data_fields {
            let new_recipe_flow_template_data_field = NewRecipeFlowTemplateDataField::new(
                &inserted_recipe_flow_template.id, 
                &rd.field_value,
                &rd.field,
                &rd.field_type,
                rd.note.as_deref(),
                &rd.required,
                rd.query.as_deref(),
                rd.default_value.as_deref()
            );
            let inserted_recipe_flow_template_data_field = diesel::insert_into(recipe_flow_template_data_fields::table)
                .values(new_recipe_flow_template_data_field)
                .get_result(conn)?;
        }
    };

}
    







