use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

use crate::{
    common::{
        agent::Agent, economic_resource::EconomicResource, location::Location, resource_specification::{ResourceSpecification, ResourceType}
    }, graphql::context::Context, recipe::recipe::RecipeWithResources, templates::{recipe_template::{RecipeTemplateType, RecipeTemplateWithRecipeFlows}, recipe_template_access::RecipeTemplateAccess}
};

use super::modules::{common::{agent, economic_resource, location, resource_specification}, recipe::recipe, templates::template::{self, RecipeFlowVisibilityFieldArg, RecipeFlowTemplateArg}};

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
        name: String,
        recipe_template_type: RecipeTemplateType,
        recipe_flow_template_args: Vec<RecipeFlowTemplateArg>,
        recipe_flow_visibility_fields: Vec<RecipeFlowVisibilityFieldArg>
    ) -> FieldResult<RecipeTemplateWithRecipeFlows> {
        template::create_recipe_template(context, name, recipe_template_type, recipe_flow_template_args, recipe_flow_visibility_fields)
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
    // fn create_process(
    //     context: &Context,
    //     recipe_id: Uuid,
    //     name: String,
    //     note: Option<String>,
    //     output_of: Option<Uuid>,
    // ) -> FieldResult<ProcessWithRecipe> {
    //     create_process(&context, recipe_id, name, note, output_of)
    // }
}
