use crate::{
    db::schema::{recipe_flow_template_data_fields, recipe_flow_templates, recipe_templates},
    graphql::context::Context,
    templates::{
        recipe_flow_template::{
            ActionType, EventType, NewRecipeFlowTemplate, RecipeFlowTemplate,
            RecipeFlowTemplateWithDataFields, RoleType,
        },
        recipe_flow_template_data_field::{
            FieldType, FieldValue, NewRecipeFlowTemplateDataField, RecipeFlowTemplateDataField, RecipeFlowTemplateDataFieldInput,
        },
        recipe_template::{
            NewRecipeTemplate, RecipeTemplate, RecipeTemplateType, RecipeTemplateWithRecipeFlows,
        },
    },
};
use diesel::prelude::*;
use juniper::{FieldError, FieldResult, ParseScalarValue};
use uuid::Uuid;

#[derive(juniper::GraphQLInputObject)]
pub struct RecipeFlowTemplateArg {
    event_type: EventType,
    role_type: RoleType,
    action: ActionType,
    data_fields: Vec<RecipeFlowTemplateDataFieldArg>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct RecipeFlowTemplateDataFieldArg {
    pub field_value: FieldValue,
    pub field: String,
    pub field_type: FieldType,
    pub note: Option<String>,
    pub required: bool,
    pub default_value: Option<String>,
}

/** Mutations */
pub fn create_recipe_template(
    context: &Context,
    name: String,
    recipe_template_type: RecipeTemplateType,
    recipe_flow_template_args: Vec<RecipeFlowTemplateArg>,
) -> FieldResult<RecipeTemplateWithRecipeFlows> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    // Create the new recipe template
    let new_template = NewRecipeTemplate::new(&name, &recipe_template_type);

    let inserted_template: RecipeTemplate = diesel::insert_into(recipe_templates::table)
        .values(&new_template)
        .get_result(conn)?;

    // Initialize the result struct
    let mut res: RecipeTemplateWithRecipeFlows =
        RecipeTemplateWithRecipeFlows::new(&inserted_template);

    // Iterate over each `RecipeFlowTemplateArg`
    for r in recipe_flow_template_args {
        // Create and insert a new recipe flow template
        let new_recipe_flow_template = NewRecipeFlowTemplate::new(
            &inserted_template.id,
            &r.event_type,
            &r.role_type,
            &r.action,
        );
        let inserted_recipe_flow_template: RecipeFlowTemplate =
            diesel::insert_into(recipe_flow_templates::table)
                .values(&new_recipe_flow_template)
                .get_result(conn)?;

        // Initialize `RecipeFlowTemplateWithDataFields` struct
        let mut recipe_flow_res =
            RecipeFlowTemplateWithDataFields::new(&inserted_recipe_flow_template);

        // Iterate over each data field and add it to the recipe flow
        for rd in r.data_fields {
            
            //TODO: build and insert query
            let query  = Uuid::new_v4();
            
            let new_recipe_flow_template_data_field = NewRecipeFlowTemplateDataField::new(
                &inserted_recipe_flow_template.id,
                &rd.field_value,
                &rd.field,
                &rd.field_type,
                rd.note.as_deref(),
                &rd.required,
                Some(&query),
                rd.default_value.as_deref(),
            );

            //Uuid, Uuid, FieldValue,     String, FieldType,    Option<String>, bool, Option<String>, Option<String>
            //Uuid, Uuid, FieldValueEnum, Text,   FieldTypeEnum, Nullable<Text>, Bool, Nullable<Text>, Nullable<Uuid>

            //TODO: fix this
            let inserted_recipe_flow_template_data_field: RecipeFlowTemplateDataField =
                diesel::insert_into(recipe_flow_template_data_fields::table)
                    .values(&new_recipe_flow_template_data_field)
                    .get_result(conn)?;

            let recipe_flow_template_data_field_input: RecipeFlowTemplateDataFieldInput =
                inserted_recipe_flow_template_data_field
                    .try_into()
                    .map_err(|e| FieldError::new(e, juniper::Value::null()))?;
                
            // Add the data field to the recipe flow
            recipe_flow_res.add_data_field(recipe_flow_template_data_field_input);
        }

        // Add the complete `recipe_flow_res` to the result
        res.add_recipe_flow(recipe_flow_res);
    }

    Ok(res)
}
