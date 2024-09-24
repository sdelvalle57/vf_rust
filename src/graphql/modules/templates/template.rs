use crate::{
    db::schema::{
        recipe_flow_template_data_fields, recipe_flow_template_group_data_fields, recipe_flow_templates, recipe_templates, recipe_templates_access
    },
    graphql::context::Context,
    templates::{
        recipe_flow_template::{
            ActionType, EventType, NewRecipeFlowTemplate, RecipeFlowTemplate,
            RecipeFlowTemplateWithDataFields, RoleType,
        }, recipe_flow_template_data_field::{
            FieldClass, FieldType, FlowThrough, NewRecipeFlowTemplateDataField,
            RecipeFlowTemplateDataField, RecipeFlowTemplateDataFieldInput,
        }, recipe_flow_template_group_data_fields::{FieldGroupClass, NewRecipeFlowTemplateGroupDataField, RecipeFlowTemplateGroupDataField}, recipe_template::{
            NewRecipeTemplate, RecipeTemplate, RecipeTemplateType, RecipeTemplateWithRecipeFlows,
        }, recipe_template_access::{NewRecipeTemplateAccess, RecipeTemplateAccess}
    },
};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use juniper::{graphql_value, FieldError, FieldResult};
use uuid::Uuid;

#[derive(juniper::GraphQLInputObject)]
pub struct RecipeFlowTemplateGroup {
    name: String,
    class: FieldGroupClass,
    fields: Vec<String>
}

#[derive(juniper::GraphQLInputObject)]
pub struct RecipeFlowTemplateArg {
    event_type: EventType,
    role_type: RoleType,
    action: ActionType,
    inherits: Option<bool>,
    data_fields: Vec<RecipeFlowTemplateDataFieldArg>,
    groups: Vec<RecipeFlowTemplateGroup>
}

#[derive(juniper::GraphQLInputObject)]
pub struct RecipeFlowTemplateDataFieldArg {
    pub field_identifier: String,
    pub field_class: FieldClass,
    pub field: String,
    pub field_type: FieldType,
    pub note: Option<String>,
    pub required: bool,
    pub flow_through: Option<FlowThrough>,
}

/** Queries */
fn get_recipe_flow_template_data_fields(
    context: &Context,
    recipe_flow_template: RecipeFlowTemplate,
) -> FieldResult<RecipeFlowTemplateWithDataFields> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let mut recipe_flow_remplate_data_fields =
        RecipeFlowTemplateWithDataFields::new(&recipe_flow_template);

    //get recipe flow template data fields by recipe flow template id
    let recipe_flow_template_data_fields: Vec<RecipeFlowTemplateDataField> =
        recipe_flow_template_data_fields::table
            .filter(
                recipe_flow_template_data_fields::recipe_flow_template_id
                    .eq(recipe_flow_template.id),
            )
            .load::<RecipeFlowTemplateDataField>(conn)?;

    for rftdf in recipe_flow_template_data_fields {
        let recipe_flow_template_data_field_input: RecipeFlowTemplateDataFieldInput = rftdf
            .try_into()
            .map_err(|e| FieldError::new(e, juniper::Value::null()))?;

        recipe_flow_remplate_data_fields.add_data_field(recipe_flow_template_data_field_input);
    }

    Ok(recipe_flow_remplate_data_fields)
}

fn get_recipe_template_with_flows(
    context: &Context,
    recipe_template: RecipeTemplate,
) -> FieldResult<RecipeTemplateWithRecipeFlows> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let mut recipe_template_with_recipe_flows: RecipeTemplateWithRecipeFlows =
        RecipeTemplateWithRecipeFlows::new(&recipe_template);

    //get recipe flow templates by recipe template id
    let recipe_flow_templates: Vec<RecipeFlowTemplate> = recipe_flow_templates::table
        .filter(recipe_flow_templates::recipe_template_id.eq(recipe_template.id))
        .load::<RecipeFlowTemplate>(conn)?;

    for rft in recipe_flow_templates {
        //create new instance of RecipeFlowTemplateWithDataFields
        let recipe_flow_remplate_data_fields = get_recipe_flow_template_data_fields(&context, rft)?;

        recipe_template_with_recipe_flows.add_recipe_flow(recipe_flow_remplate_data_fields)
    }

    Ok(recipe_template_with_recipe_flows)
}

pub fn get_templates(context: &Context) -> FieldResult<Vec<RecipeTemplateWithRecipeFlows>> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let mut res: Vec<RecipeTemplateWithRecipeFlows> = Vec::new();

    //Get all recipe templates
    let recipe_templates: Vec<RecipeTemplate> =
        recipe_templates::table.load::<RecipeTemplate>(conn)?;

    for rt in recipe_templates {
        //create instance of RecipeTemplateWithRecipeFlows
        let recipe_template_with_recipe_flows = get_recipe_template_with_flows(&context, rt)?;

        res.push(recipe_template_with_recipe_flows);
    }

    Ok(res)
}

pub fn get_template_by_id(
    context: &Context,
    template_id: Uuid,
) -> FieldResult<RecipeTemplateWithRecipeFlows> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let recipe: RecipeTemplate = recipe_templates::table
        .filter(recipe_templates::id.eq(template_id))
        .first::<RecipeTemplate>(conn)?;

    let res = get_recipe_template_with_flows(&context, recipe)?;

    Ok(res)
}

pub fn get_templates_access_by_agent(
    context: &Context,
    agent_id: Uuid,
) -> FieldResult<Vec<RecipeTemplateWithRecipeFlows>> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let accesses: Vec<RecipeTemplateAccess> = recipe_templates_access::table
        .filter(recipe_templates_access::agent_id.eq(agent_id))
        .load::<RecipeTemplateAccess>(conn)?;

    let mut res: Vec<RecipeTemplateWithRecipeFlows> = Vec::new();

    for a in accesses {
        let template_id = a.recipe_template_id;
        let recipe = get_template_by_id(context, template_id)?;
        res.push(recipe)
    }

    Ok(res)
}

/*
commitment -> Nullable<ActionTypeEnum>,
        fulfills -> Nullable<Uuid>,
*/
pub fn create_recipe_template(
    context: &Context,
    identifier: String,
    name: String,
    recipe_template_type: RecipeTemplateType,
    recipe_flow_template_args: Vec<RecipeFlowTemplateArg>,
    commitment: Option<ActionType>,
    fulfills: Option<String>,
) -> FieldResult<RecipeTemplateWithRecipeFlows> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    // Start a transaction
    conn.transaction::<_, FieldError, _>(|conn| {
        // Create the new recipe template
        let fulfills_id = if let Some(process_identifier) = fulfills {
            // Attempt to get the recipe by identifier
            let recipe: RecipeTemplate = recipe_templates::table
                .filter(recipe_templates::identifier.eq(process_identifier))
                .first::<RecipeTemplate>(conn)?;
        
            // Return the recipe's id wrapped in Some
            Some(recipe.id)
        } else {
            // If fulfills is None, return None
            None
        };
        
        let new_template = NewRecipeTemplate::new(
            &identifier,
            &name, 
            commitment.as_ref(),
            fulfills_id.as_ref(),
            &recipe_template_type
            
        );

        let inserted_template: RecipeTemplate = diesel::insert_into(recipe_templates::table)
            .values(&new_template)
            .get_result(conn)
            .map_err(|e| FieldError::new(e, juniper::Value::null()))?; // Map diesel::result::Error to FieldError

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
                r.inherits.as_ref(),
                &r.action,
            );
            
            let inserted_recipe_flow_template: RecipeFlowTemplate = diesel::insert_into(recipe_flow_templates::table)
                .values(&new_recipe_flow_template)
                .get_result::<RecipeFlowTemplate>(conn)?;

            // Initialize `RecipeFlowTemplateWithDataFields` struct
            let mut recipe_flow_res =
                RecipeFlowTemplateWithDataFields::new(&inserted_recipe_flow_template);

            let mut groups: Vec<(Uuid, Vec<String>)> = Vec::new();

            for group in r.groups {

                let new_group = NewRecipeFlowTemplateGroupDataField::new(&group.name, &group.class);

                let inserted_group: RecipeFlowTemplateGroupDataField = diesel::insert_into(recipe_flow_template_group_data_fields::table)
                    .values(new_group)
                    .get_result::<RecipeFlowTemplateGroupDataField>(conn)?;

                groups.push((inserted_group.id, group.fields));
            }

            // Iterate over each data field and add it to the recipe flow
            for rd in r.data_fields {
                let flow_through_ref: Option<&FlowThrough> = rd.flow_through.as_ref();

                //search in groups if the field identifier is the groups.1 vector and return the group_id
                let group_id = groups.iter().find(|g| {
                    g == g
                });

                let new_recipe_flow_template_data_field = NewRecipeFlowTemplateDataField::new(
                    &inserted_recipe_flow_template.id,
                    None,
                    &rd.field_identifier,
                    &rd.field_class,
                    &rd.field,
                    &rd.field_type,
                    rd.note.as_deref(),
                    &rd.required,
                    flow_through_ref,
                );

                let inserted_recipe_flow_template_data_field: RecipeFlowTemplateDataField =
                    diesel::insert_into(recipe_flow_template_data_fields::table)
                        .values(new_recipe_flow_template_data_field)
                        .get_result(conn)
                        .map_err(|e| FieldError::new(e, juniper::Value::null()))?; // Map diesel::result::Error to FieldError

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

        // If all succeeds, return the result
        Ok(res)
    })
}

pub fn assign_template_to_agent(
    context: &Context,
    recipe_template_id: Uuid,
    agent_id: Uuid,
) -> FieldResult<RecipeTemplateAccess> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let existing_access: Result<RecipeTemplateAccess, DieselError> = recipe_templates_access::table
        .filter(recipe_templates_access::recipe_template_id.eq(recipe_template_id))
        .filter(recipe_templates_access::agent_id.eq(agent_id))
        .first::<RecipeTemplateAccess>(conn);

    // If it exists, return an error
    if let Ok(_) = existing_access {
        return Err(FieldError::new(
            "Template access already exists for this agent.",
            graphql_value!({ "code": "ALREADY_EXISTS" }),
        ));
    }

    // If it doesn't exist, insert a new record
    let new_access = NewRecipeTemplateAccess::new(&agent_id, &recipe_template_id);

    let res: RecipeTemplateAccess = diesel::insert_into(recipe_templates_access::table)
        .values(&new_access)
        .get_result::<RecipeTemplateAccess>(conn)?;

    Ok(res)
}
