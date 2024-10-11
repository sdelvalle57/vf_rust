use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

use crate::{
    common::{
        agent::Agent, economic_resource::EconomicResource, location::Location, resource_specification::{ResourceSpecification, ResourceType}
    }, graphql::context::Context, recipe::recipe::RecipeWithResources, templates::{map_template::{MapTemplate, TemplateType}, recipe_flow_template::ActionType, recipe_template::RecipeTemplateWithRecipeFlows, recipe_template_access::RecipeTemplateAccess}
};

use super::modules::{
    common::{agent, economic_resource, location, resource_specification}, 
    // process::process::{self, CreateRecipeProcessesResponse, ProcessExecution, RecipeProcessWithRelation}, 
    recipe::recipe, templates::template::{self, RecipeFlowTemplateArg}
};

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    /*** Agents */
    fn create_agent(context: &Context, name: String, note: Option<String>) -> FieldResult<Agent> {
        agent::create_agent(&context, name, note)
    }

    /** Resource Specifications */
    fn create_resource_specification(
        context: &Context,
        agent_id: Uuid,
        name: String,
        note: Option<String>,
        resource_type: ResourceType,
        unit_of_measure: String,
    ) -> FieldResult<ResourceSpecification> {
        resource_specification::create_resource_specification(
            &context,
            agent_id,
            name,
            note,
            resource_type,
            unit_of_measure,
        )
    }

    /** Economic Resource */
    fn create_economic_resource(
        context: &Context,
        resource_specification_id: Uuid,
        name: String,
        note: Option<String>,
        accounting_quantity: i32,
        tracking_identifier: Option<String>,
        current_location: String,
        lot: Option<String>,
        contained_in: Option<Uuid>,
    ) -> FieldResult<EconomicResource> {
        economic_resource::create_economic_resource(
            &context,
            resource_specification_id,
            name,
            note,
            accounting_quantity,
            tracking_identifier,
            current_location,
            lot,
            contained_in,
        )
    }

    /** Map Templates */
    fn create_map_template(
        context: &Context,
        name: String,
        type_: TemplateType
    ) -> FieldResult<MapTemplate> {
        template::create_map_template(
            context, 
            name, 
            type_
        )
    }

    /** Recipe Templates */
    fn create_recipe_template(
        context: &Context,
        map_template_id: Uuid,
        identifier: String,
        name: String,
        recipe_flow_template_args: Vec<RecipeFlowTemplateArg>,
        commitment: Option<ActionType>,
        fulfills: Option<String>,
        trigger: Option<ActionType>
    ) -> FieldResult<RecipeTemplateWithRecipeFlows> {
        template::create_recipe_template(
            context, 
            map_template_id,
            identifier,
            name, 
            recipe_flow_template_args,
            commitment,
            fulfills,
            trigger
        )
    }

    /** Recipe Template Access */
    fn assign_template_to_agent(
        context: &Context,
        recipe_template_id: Uuid,
        agent_id: Uuid,
    ) -> FieldResult<RecipeTemplateAccess> {
        template::assign_template_to_agent(context, recipe_template_id, agent_id)
    }

    /** Recipe */
    fn create_recipe(
        context: &Context,
        agent_id: Uuid,
        name: String,
        note: Option<String>,
        recipe_resources: Vec<Uuid>,
    ) -> FieldResult<RecipeWithResources> {
        recipe::create_recipe(&context, agent_id, name, note, recipe_resources)
    }

    /** Locations */
    fn create_location(context: &Context, agent_id: Uuid, name: String, value: String) -> FieldResult<Location> {
        location::create_location(&context, agent_id, name, value)
    }

    // /** Process */
    // fn create_recipe_processes(
    //     context: &Context,
    //     recipe_id: Uuid,
    //     data: Vec<RecipeProcessWithRelation>
    // ) -> FieldResult<CreateRecipeProcessesResponse> {
    //     process::create_recipe_processes(&context, recipe_id, data)
    // }

    // /** Process Execution */
    // fn execute_events(context: &Context, recipe_process_id: Uuid, process_flows: Vec<ProcessExecution>) -> FieldResult<String> {
    //     process::execute_events(&context, recipe_process_id, process_flows)
    // }
}





/*
fn get_latest_lot_code_for_agent(conn: &PgConnection, agent_id: Uuid) -> QueryResult<(Option<i32>, Option<i32>)> {
    use crate::schema::lot_codes::dsl::*;

    lot_codes
        .filter(agent_id.eq(agent_id))
        .select((diesel::dsl::max(lot_code), diesel::dsl::max(reference_number)))
        .first::<(Option<i32>, Option<i32>)>(conn)
}

        fn insert_lot_code(
    conn: &PgConnection,
    agent: Uuid,
    increment_lot_code: bool,
    increment_reference_number: bool,
) -> QueryResult<LotCode> {
    use crate::schema::lot_codes;
    use diesel::insert_into;

    // Get the latest lot_code and reference_number for the agent
    let (latest_lot_code, latest_reference_number) = get_latest_lot_code_for_agent(conn, agent)?;

    // Determine the new values for lot_code and reference_number
    let new_lot_code = if increment_lot_code {
        latest_lot_code.unwrap_or(0) + 1
    } else {
        latest_lot_code.unwrap_or(1) // Default to 1 if none found
    };

    let new_reference_number = if increment_reference_number {
        latest_reference_number.unwrap_or(0) + 1
    } else {
        latest_reference_number.unwrap_or(1) // Default to 1 if none found
    };

    // Insert the new row into the table
    let new_lot_code_row = diesel::insert_into(lot_codes::table)
        .values((
            lot_codes::agent_id.eq(agent),
            lot_codes::lot_code.eq(new_lot_code),
            lot_codes::reference_number.eq(new_reference_number),
        ))
        .get_result::<LotCode>(conn)?;

    Ok(new_lot_code_row)
}


// Example 1: Increment only the reference number
let new_lot_code = insert_lot_code(&conn, agent_id, false, true)
    .expect("Error inserting new lot code");

// Example 2: Increment the lot code and the reference number
let new_lot_code = insert_lot_code(&conn, agent_id, true, true)
    .expect("Error inserting new lot code");

*/