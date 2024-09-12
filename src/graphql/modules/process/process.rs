use diesel::prelude::*;
use juniper::{FieldError, FieldResult, GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::{
    common::resource_specification::ResourceSpecification, db::schema::{
        recipe_process_flow_data_fields, recipe_process_flows, recipe_processes, recipe_resources,
        recipes,
    }, graphql::{context::Context, modules::common::resource_specification::resource_specification_by_id}, recipe::{
        process::{
            data_field::{NewRecipeFlowDataField, RecipeFlowDataField},
            flow::{NewRecipeProcessFlow, RecipeProcessFlow, RecipeProcessFlowResponse},
            process::{NewRecipeProcess, RecipeProcess, RecipeProcessResponse},
        },
        recipe::{Recipe, RecipeResource},
    }, templates::{
        recipe_flow_template::{ActionType, EventType, RoleType},
        recipe_flow_template_data_field::{FieldClass, FieldType, FlowThrough},
        recipe_template::RecipeTemplateType,
    }
};

#[derive(GraphQLInputObject)]
pub struct RecipeProcessWithRelation {
    pub recipe_process: RecipeWithRecipeFlows,
    pub output_of: Option<RecipeWithRecipeFlows>,
}

#[derive(GraphQLInputObject)]
pub struct RecipeWithRecipeFlows {
    pub id: Uuid,
    pub name: String,
    pub recipe_template_type: RecipeTemplateType,
    pub recipe_flows: Vec<RecipeFlowWithDataFields>,
}

#[derive(GraphQLInputObject)]
pub struct RecipeFlowWithDataFields {
    pub id: Uuid,
    pub recipe_template_id: Uuid,
    pub event_type: EventType,
    pub role_type: RoleType,
    pub action: ActionType,
    pub data_fields: Vec<RecipeFlowDataFieldInput>,
}

#[derive(GraphQLInputObject)]
pub struct RecipeFlowDataFieldInput {
    pub id: Option<Uuid>,
    pub field_class: FieldClass,
    pub field: String,
    pub field_type: FieldType,
    pub note: Option<String>,
    pub required: bool,
    pub field_identifier: String,
    pub flow_through: Option<FlowThrough>,
    pub default_value: Option<String>,
}

#[derive(GraphQLObject)]
pub struct CreateRecipeProcessesResponse {
    pub recipe: Recipe,
    pub resources: Vec<ResourceSpecification>,
    pub recipe_processes: Vec<RecipeProcessResponse>,
}

impl CreateRecipeProcessesResponse {
    pub fn new(recipe: Recipe, resources: Vec<ResourceSpecification>) -> Self {
        CreateRecipeProcessesResponse {
            recipe,
            resources,
            recipe_processes: Vec::new(),
        }
    }

    pub fn add_recipe_process(&mut self, recipe_process: RecipeProcessResponse) {
        self.recipe_processes.push(recipe_process);
    }
}

#[derive(GraphQLObject)]
pub struct RecipeProcessesResponse {
    pub recipe: Recipe,
    pub resources: Vec<ResourceSpecification>,
    pub recipe_processes: Vec<RecipeProcessResponse>,
}

impl RecipeProcessesResponse {
    pub fn new(recipe: Recipe, resources: Vec<ResourceSpecification>) -> Self {
        RecipeProcessesResponse {
            recipe,
            resources,
            recipe_processes: Vec::new(),
        }
    }

    pub fn add_recipe_process(&mut self, recipe_process: RecipeProcessResponse) {
        self.recipe_processes.push(recipe_process);
    }
}

/** Queries */
pub fn get_recipe_processes(
    context: &Context,
    recipe_id: Uuid,
) -> FieldResult<RecipeProcessesResponse> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let recipe: Recipe = recipes::table
        .filter(recipes::id.eq(recipe_id))
        .first::<Recipe>(conn)?;

    let recipe_resources: Vec<RecipeResource> = recipe_resources::table
        .filter(recipe_resources::recipe_id.eq(recipe_id))
        .load::<RecipeResource>(conn)?;

    let mut resources: Vec<ResourceSpecification> = Vec::new();

    for resource in recipe_resources {
        let spec = resource_specification_by_id(context, resource.resource_specification_id)?;
        resources.push(spec)
    };

    let mut res: RecipeProcessesResponse =
        RecipeProcessesResponse::new(recipe.clone(), resources);

    let recipe_processes: Vec<RecipeProcess> = recipe_processes::table
        .filter(recipe_processes::recipe_id.eq(recipe_id))
        .load::<RecipeProcess>(conn)?;

    for recipe_process in recipe_processes {
        let mut recipe_process_response: RecipeProcessResponse =
            RecipeProcessResponse::new(recipe_process.clone());

        let recipe_process_flows: Vec<RecipeProcessFlow> = recipe_process_flows::table
            .filter(recipe_process_flows::recipe_process_id.eq(recipe_process.id))
            .load::<RecipeProcessFlow>(conn)?;

        for recipe_process_flow in recipe_process_flows {
            let mut recipe_process_flow_response =
                RecipeProcessFlowResponse::new(recipe_process_flow.clone());

            let recipe_process_flow_data_fields: Vec<RecipeFlowDataField> =
                recipe_process_flow_data_fields::table
                    .filter(
                        recipe_process_flow_data_fields::recipe_process_flow_id
                            .eq(recipe_process_flow.id),
                    )
                    .load::<RecipeFlowDataField>(conn)?;

            for recipe_process_flow_data_field in recipe_process_flow_data_fields {
                recipe_process_flow_response.add_data_field(recipe_process_flow_data_field);
            }

            recipe_process_response.add_recipe_process_flow(recipe_process_flow_response);
        }

        res.add_recipe_process(recipe_process_response);
    }

    Ok(res)
}

/** Mutations */
pub fn create_recipe_processes(
    context: &Context,
    recipe_id: Uuid,
    data: Vec<RecipeProcessWithRelation>,
) -> FieldResult<CreateRecipeProcessesResponse> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    conn.transaction::<_, FieldError, _>(|conn| {
        let recipe: Recipe = recipes::table
            .filter(recipes::id.eq(recipe_id))
            .first::<Recipe>(conn)?;

        let recipe_resources: Vec<RecipeResource> = recipe_resources::table
            .filter(recipe_resources::recipe_id.eq(recipe_id))
            .load::<RecipeResource>(conn)?;
    
        let mut resources: Vec<ResourceSpecification> = Vec::new();
    
        for resource in recipe_resources {
            let spec = resource_specification_by_id(context, resource.resource_specification_id)?;
            resources.push(spec)
        };

        let mut res: CreateRecipeProcessesResponse =
            CreateRecipeProcessesResponse::new(recipe.clone(), resources);

        for recipe_process in data {
            let output_of_id: Option<&Uuid> =
                recipe_process.output_of.as_ref().map(|output| &output.id);

            //search for recipe_process_templates with id output_of_id
            let output = if let Some(id) = output_of_id {
                //search for same info saved into recipe_processes
                let output: RecipeProcess = recipe_processes::table
                    .filter(recipe_processes::recipe_id.eq(recipe.id))
                    .filter(recipe_processes::recipe_template_id.eq(id))
                    .first(conn)?;

                Some(output.id)
            } else {
                None
            };

            let output_of_id: Option<&Uuid> = output.as_ref();

            let new_recipe_process = NewRecipeProcess::new(
                &recipe_id,
                &recipe_process.recipe_process.id,
                &recipe_process.recipe_process.name,
                &recipe_process.recipe_process.recipe_template_type,
                output_of_id,
            );

            let inserted_recipe_process: RecipeProcess =
                diesel::insert_into(recipe_processes::table)
                    .values(new_recipe_process)
                    .get_result(conn)?;

            let mut recipe_process_response: RecipeProcessResponse =
                RecipeProcessResponse::new(inserted_recipe_process.clone());

            //Iterate over flows
            for flow in recipe_process.recipe_process.recipe_flows {
                let new_recipe_flow = NewRecipeProcessFlow::new(
                    &inserted_recipe_process.id,
                    &flow.id,
                    &flow.event_type,
                    &flow.role_type,
                    &flow.action,
                );

                let inserted_recipe_flow: RecipeProcessFlow =
                    diesel::insert_into(recipe_process_flows::table)
                        .values(new_recipe_flow)
                        .get_result(conn)?;

                let mut recipe_process_flow_response =
                    RecipeProcessFlowResponse::new(inserted_recipe_flow.clone());

                //Iterate over data fields, if data field comes from recipe_flow_template_data_fields so data_field_id should be defined
                for data_field in flow.data_fields {
                    let data_field_id: Option<&Uuid> = data_field.id.as_ref();

                    let flow_through_ref: Option<&FlowThrough> = data_field.flow_through.as_ref();

                    //check this, check on default value
                    let new_data_field = NewRecipeFlowDataField::new(
                        &inserted_recipe_flow.id,
                        data_field_id, // Wrap Uuid in Some if necessary
                        &data_field.field_identifier,
                        &data_field.field_class,
                        &data_field.field,
                        &data_field.field_type,
                        data_field.note.as_deref(), // Safely handle Option<&str> for note
                        data_field.required,
                        data_field.default_value.as_deref(), // Convert Uuid to String if needed
                        flow_through_ref,
                    );

                    let inserted_data_field: RecipeFlowDataField =
                        diesel::insert_into(recipe_process_flow_data_fields::table)
                            .values(new_data_field)
                            .get_result(conn)?;

                    println!(
                        "{} {:?} {}",
                        recipe_process.recipe_process.name,
                        flow.action,
                        data_field.field_identifier
                    );

                    recipe_process_flow_response.add_data_field(inserted_data_field);
                }

                recipe_process_response.add_recipe_process_flow(recipe_process_flow_response);
            }

            res.add_recipe_process(recipe_process_response)
        }

        //diesel::sql_types::Uuid, diesel::sql_types::Uuid, diesel::sql_types::Nullable<diesel::sql_types::Uuid>, diesel::sql_types::Text, FieldClassEnum, diesel::sql_types::Text, FieldTypeEnum, diesel::sql_types::Nullable<diesel::sql_types::Text>, diesel::sql_types::Bool, diesel::sql_types::Nullable<diesel::sql_types::Text>, diesel::sql_types::Nullable<FlowThroughEnum>
        Ok(res)
    })
}
