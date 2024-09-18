use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

use crate::{
    common::{
        agent::Agent, economic_resource::EconomicResource, location::Location, resource_specification::{ResourceSpecification, ResourceType}
    }, graphql::context::Context, recipe::recipe::RecipeWithResources, templates::{recipe_flow_template::ActionType, recipe_template::{RecipeTemplateType, RecipeTemplateWithRecipeFlows}, recipe_template_access::RecipeTemplateAccess}
};

use super::modules::{common::{agent, economic_resource, location, resource_specification}, process::process::{create_recipe_processes, CreateRecipeProcessesResponse, RecipeProcessWithRelation}, recipe::recipe, templates::template::{self, RecipeFlowTemplateArg}};

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

    /** Recipe Templates */
    fn create_recipe_template(
        context: &Context,
        identifier: String,
        name: String,
        recipe_template_type: RecipeTemplateType,
        recipe_flow_template_args: Vec<RecipeFlowTemplateArg>,
        commitment: Option<ActionType>,
        fulfills: Option<String>,
    ) -> FieldResult<RecipeTemplateWithRecipeFlows> {
        template::create_recipe_template(
            context, 
            identifier,
            name, 
            recipe_template_type, 
            recipe_flow_template_args,
            commitment,
            fulfills
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
    fn create_recipe_processes(
        context: &Context,
        recipe_id: Uuid,
        data: Vec<RecipeProcessWithRelation>
    ) -> FieldResult<CreateRecipeProcessesResponse> {
        create_recipe_processes(&context, recipe_id, data)
    }
}
