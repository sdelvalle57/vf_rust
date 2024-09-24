use std::fmt::format;

use diesel::{dsl::Field, prelude::*};
use juniper::{graphql_value, FieldError, FieldResult, GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::{
    common::resource_specification::ResourceSpecification,
    db::schema::{
        recipe_process_flow_data_fields, recipe_process_flows, recipe_process_relations,
        recipe_processes, recipe_resources, recipe_templates, recipes,
    },
    graphql::{
        context::Context, modules::common::resource_specification::resource_specification_by_id,
    },
    recipe::{
        process::{
            data_field::{NewRecipeFlowDataField, RecipeFlowDataField},
            flow::{NewRecipeProcessFlow, RecipeProcessFlow, RecipeProcessFlowResponse},
            process::{
                NewOutpuOf, NewRecipeProcess, OutputOf, ProcessFlow, RecipeProcess,
                RecipeProcessResponse,
            },
        },
        recipe::{Recipe, RecipeResource},
    },
    templates::{
        recipe_flow_template::{ActionType, EventType, RoleType},
        recipe_flow_template_data_field::{FieldClass, FieldType, FlowThrough},
        recipe_template::{RecipeTemplate, RecipeTemplateType},
    },
};

#[derive(GraphQLInputObject)]
pub struct RecipeProcessWithRelation {
    pub recipe_process: RecipeWithRecipeFlows,
    pub output_of: Vec<RecipeWithRecipeFlows>,
}

#[derive(GraphQLInputObject)]
pub struct RecipeWithRecipeFlows {
    pub id: Uuid,
    pub name: String,
    pub recipe_template_type: RecipeTemplateType,
    pub commitment: Option<ActionType>,
    pub fulfills: Option<Uuid>,
    pub recipe_flows: Vec<RecipeFlowWithDataFields>,
    pub identifier: String,
}

#[derive(GraphQLInputObject)]
pub struct RecipeFlowWithDataFields {
    pub id: Uuid,
    pub recipe_template_id: Uuid,
    pub event_type: EventType,
    pub role_type: RoleType,
    pub action: ActionType,
    pub inherits: Option<bool>,
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
    }

    let mut res: RecipeProcessesResponse = RecipeProcessesResponse::new(recipe.clone(), resources);

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

        let output_of_values: Vec<OutputOf> = recipe_process_relations::table
            .filter(recipe_process_relations::recipe_process_id.eq(recipe_process.id))
            .load::<OutputOf>(conn)?;

        for output_of in output_of_values {
            recipe_process_response.add_output_of(output_of.id);
        }

        res.add_recipe_process(recipe_process_response);
    }

    Ok(res)
}

/** Mutations */
//TODO: should check for inheritance, commitment and fulfillment
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
        }

        let mut res: CreateRecipeProcessesResponse =
            CreateRecipeProcessesResponse::new(recipe.clone(), resources);

        for recipe_process in data {
            let fulfills: Option<Uuid> =
                if let Some(fulfills_value) = recipe_process.recipe_process.fulfills {
                    //get from recipe_templates the data that fulfills this
                    //get the identifier
                    let recipe_process_template: RecipeTemplate = recipe_templates::table
                        .filter(recipe_templates::id.eq(fulfills_value))
                        .first::<RecipeTemplate>(conn)?;

                    //search for the identifier in the recipe_process table
                    let recipe_process: RecipeProcess = recipe_processes::table
                        .filter(recipe_processes::recipe_id.eq(recipe_id))
                        .filter(recipe_processes::identifier.eq(recipe_process_template.identifier))
                        .first(conn)?;

                    Some(recipe_process.id)
                } else {
                    None
                };

            let new_recipe_process = NewRecipeProcess::new(
                &recipe_id,
                &recipe_process.recipe_process.id,
                &recipe_process.recipe_process.name,
                recipe_process.recipe_process.commitment.as_ref(),
                fulfills.as_ref(),
                &recipe_process.recipe_process.recipe_template_type,
                &recipe_process.recipe_process.identifier,
            );

            let inserted_recipe_process: RecipeProcess =
                diesel::insert_into(recipe_processes::table)
                    .values(new_recipe_process)
                    .get_result(conn)?;

            let mut recipe_process_response: RecipeProcessResponse =
                RecipeProcessResponse::new(inserted_recipe_process.clone());

            for output_of in recipe_process.output_of {
                let process_output_of: RecipeProcess = recipe_processes::table
                    .filter(recipe_processes::recipe_id.eq(&recipe_id))
                    .filter(recipe_processes::recipe_template_id.eq(output_of.id))
                    .first::<RecipeProcess>(conn)?;

                let new_output_of =
                    NewOutpuOf::new(&inserted_recipe_process.id, &process_output_of.id);

                let inserted_relation: OutputOf =
                    diesel::insert_into(recipe_process_relations::table)
                        .values(new_output_of)
                        .get_result(conn)?;

                recipe_process_response.add_output_of(inserted_relation.output_of);
            }

            //Iterate over flows
            for flow in recipe_process.recipe_process.recipe_flows {
                let new_recipe_flow = NewRecipeProcessFlow::new(
                    &inserted_recipe_process.id,
                    &flow.id,
                    &flow.event_type,
                    &flow.role_type,
                    &flow.action,
                    flow.inherits.as_ref(),
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
                        "{} {:?} {} {:?}",
                        recipe_process.recipe_process.name,
                        flow.action,
                        data_field.field_identifier,
                        flow.role_type
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

#[derive(GraphQLInputObject, Debug)]
pub struct DataFieldValue {
    id: Uuid,
    value: String,
}

#[derive(GraphQLInputObject)]
pub struct ProcessExecution {
    process_flow_id: Uuid,
    data_field_values: Vec<DataFieldValue>,
}

//TODO: iterate over process_flows, get process flow info from db, get action and role_type
//check action of the process flow, to apply the logic depending on action and role_type
//also iterate over data fields and complete the info to create the data in process_executions
//and process_execution_custom_values tables
pub fn execute_events(
    context: &Context,
    recipe_process_id: Uuid,
    process_flows: Vec<ProcessExecution>,
) -> FieldResult<String> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    // Begin transaction
    conn.transaction::<_, FieldError, _>(|conn| {
        //get the recipe process
        let recipe_process: RecipeProcess = recipe_processes::table
            .filter(recipe_processes::id.eq(recipe_process_id))
            .first::<RecipeProcess>(conn)?;

        if let Some(fulfills) = recipe_process.fulfills {
            execute_fulfillment(context, fulfills)?;
        }

        for process_flow in process_flows {
            let process_flow_id = process_flow.process_flow_id;
            let process_flow_info: ProcessFlow = recipe_process_flows::table
                .filter(recipe_process_flows::recipe_process_id.eq(recipe_process_id))
                .filter(recipe_process_flows::id.eq(process_flow_id))
                .first::<ProcessFlow>(conn)?;

            let action = process_flow_info.action;
            let role_type = process_flow_info.role_type;
            let error_message = format!("Invalid Action {:?}", action.clone());

            let process_flow_data_fields: Vec<RecipeFlowDataField> =
                recipe_process_flow_data_fields::table
                    .filter(
                        recipe_process_flow_data_fields::recipe_process_flow_id.eq(process_flow_id),
                    )
                    .load::<RecipeFlowDataField>(conn)?;

            // Handle invalid action based on role_type
            if let RoleType::Input = role_type {
                match action {
                    _ => {
                        return Err(FieldError::new(
                            "Action is not valid as Output",
                            graphql_value!({ "code": error_message }),
                        ));
                    }
                }
            } else if let RoleType::Output = role_type {
                match action {
                    ActionType::Produce => {
                        let product_field_value = get_field_value(
                            &process_flow,
                            &process_flow_data_fields,
                            FieldClass::ResourceSpecification,
                        )?;
                        println!("Product {:?}", product_field_value.1);

                        let quantity_field_value = get_field_value(
                            &process_flow,
                            &process_flow_data_fields,
                            FieldClass::Quantity,
                        )?;
                        println!("Quantity {:?}", quantity_field_value.1);

                        let extra_fields = get_extra_fields(&process_flow, &process_flow_data_fields)?;
                       
                        for extra_field in extra_fields {
                            println!("{:?} {:?}", extra_field.field_class, extra_field.value);
                        };
                    }
                    _ => {
                        return Err(FieldError::new(
                            "Action is not valid as Output",
                            graphql_value!({ "code": error_message }),
                        ));
                    }
                }
            }
        }
        Ok(())
    })?;

    // If the transaction is successful, return the result
    Ok(format!("All events executed successfully"))
}

fn execute_fulfillment(context: &Context, fulfills: Uuid) -> FieldResult<bool> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let recipe_process_result = recipe_processes::table
        .filter(recipe_processes::id.eq(fulfills))
        .first::<RecipeProcess>(conn);

    // Check if the query was successful
    match recipe_process_result {
        Ok(res) => {
            match res.commitment {
                Some(commitment) => {
                    match commitment {
                        ActionType::Transfer => {
                            //get the product and quantity
                            Ok(true)
                        }
                        _ => {
                            let error_message =
                                format!("Commitment action not supported {:?}", res.name);
                            Err(FieldError::new(
                                "Unable to execute fulfillment",
                                graphql_value!({ "code": error_message }),
                            ))
                        }
                    }
                }
                None => {
                    let error_message =
                        format!("Unable to find commitment action in {:?}", res.name);
                    Err(FieldError::new(
                        "Unable to execute fulfillment",
                        graphql_value!({ "code": error_message }),
                    ))
                }
            }
        } // If the query succeeds, return true
        Err(e) => {
            let error_message = format!("Commitment not found {:?}", e);
            Err(FieldError::new(
                "Unable to execute fulfillment",
                graphql_value!({ "code": error_message }),
            ))
        }
    }
}

fn get_field_value(
    process_flow: &ProcessExecution,
    data_fields: &Vec<RecipeFlowDataField>,
    field_class: FieldClass,
) -> FieldResult<(RecipeFlowDataField, String)> {
    // Find the product field
    let product_field = if let Some(product_field) = data_fields
        .iter()
        .find(|field| field.field_class == field_class)
    {
        (*product_field).clone() // Dereference the reference and then clone
    } else {
        let error_message = format!("Cannot Find Product {:?} Data Field", field_class); 
        return Err(FieldError::new(
            "Unable to find Data Field",
            graphql_value!({ "code": error_message }),
        ));
    };

    // Find the field value
    let field_value = if let Some(field_value) = process_flow
        .data_field_values
        .iter()
        .find(|df| df.id == product_field.id)
    {
        field_value.value.clone()
    } else {
        let error_message = format!("Cannot Find Product {:?} Data Field", field_class); 
        return Err(FieldError::new(
            "Unable to find Data Field Value",
            graphql_value!({ "code": error_message }),
        ));
    };

    // Return the cloned product field and field value
    Ok((product_field, field_value))
}

#[derive(Debug)]
struct ExtraField {
    field: RecipeFlowDataField,
    field_class: FieldClass,
    value: String,
}

fn get_extra_fields(
    process_flow: &ProcessExecution,
    data_fields: &Vec<RecipeFlowDataField>,
) -> FieldResult<Vec<ExtraField>> {
    let mut res = Vec::new();
    
    let location_val = get_field(&process_flow, &data_fields, FieldClass::Location)?;
    let has_point_in_time_val = get_field(&process_flow, &data_fields, FieldClass::HasPointInTime)?;
    let note_val = get_field(&process_flow, &data_fields, FieldClass::Note)?;
    
    if let Some(location) = location_val {
        res.push(location);
    };

    if let Some(has_point_in_time) = has_point_in_time_val {
        res.push(has_point_in_time);
    };

    if let Some(note) = note_val {
        res.push(note);
    }
    
    Ok(res)
}

fn get_custom_fields(
    process_flow: &ProcessExecution,
    data_fields: &Vec<RecipeFlowDataField>
) -> FieldResult<Vec<ExtraField>> {
    
    Ok(Vec::new())
}

fn get_field(
    process_flow: &ProcessExecution,
    data_fields: &Vec<RecipeFlowDataField>,
    field_class: FieldClass,
) -> FieldResult<Option<ExtraField>> {
    if let Some(product_field) = data_fields
        .iter()
        .find(|field| field.field_class == field_class) {
        
        let field =(*product_field).clone();
        
        let value = if let Some(field_value) = process_flow
            .data_field_values
            .iter()
            .find(|df| df.id == product_field.id) {
            field_value.value.clone()
        } else {
            let error_message = format!("Field Value {:?} Not Found ", field_class);
            return Err(FieldError::new(
                "Unable to find Data Field Value",
                graphql_value!({ "code": error_message }),
            ));
        };

        let res: ExtraField = ExtraField {
            field_class,
            field,
            value
        };
        return Ok(Some(res));
    };
    Ok(None)

}
